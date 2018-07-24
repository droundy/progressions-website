from jinja2 import Environment, FileSystemLoader
import os, csv, slugify, glob, copy, markdown
import hashlib

def file_hash(filename):
  h = hashlib.sha256()
  with open(filename, 'rb', buffering=0) as f:
    for b in iter(lambda : f.read(128*1024), b''):
      h.update(b)
  return h.hexdigest()

env = Environment(
    loader=FileSystemLoader('templates'),
)

style_css = file_hash('style.css')[:8] + '.css'
os.system('mkdir -p output')
os.system('rm -f output/*.css')
os.system('cp -v style.css output/' + style_css)
os.system('cp -v style.css output/')

course_template = env.get_template('course.html')

activities = []
concepts = []

class Course:
    """ A course """
    __p = {}
    def __init__(self, number, name=None):
        if number not in Course.__p:
            self.__p[number] = {
                'number': number,
                'name': name,
                'activities': [],
                'concepts': [],
                'orphan_concepts': [],
            }
            if name is not None:
                Course.__p[name] = self.__p[number] # can look up Courses either way
        self.number = self.__p[number]['number'] # in case we were given name instead!
        if name is not None:
            self.__p[number]['name'] = name
    def __repr__(self):
        return 'Course(%s)' % self.number
    def __eq__(self, other):
        return other is not None and self.number == other.number
    def __ne__(self, other):
        return other is None or self.number != other.number
    def __hash__(self):
        return hash(self.number)
    @property
    def name(self):
        return self.__p[self.number]['name']
    @property
    def activities(self):
        return self.__p[self.number]['activities']
    @property
    def concepts(self):
        return self.__p[self.number]['concepts']
    @property
    def orphan_concepts(self):
        return self.__p[self.number]['orphan_concepts']

class Activity:
    """ A teaching activity """
    __p = {}
    def __init__(self, name, course=None, prereqs=[], concepts=[], representations=[],
                 description='', rownum=None, url=None, figure=None):
        self.name = name
        if name not in self.__p:
            self.__p[name] = {
                'name': name,
                'course': None,
                'prereqs': [],
                'concepts': [],
                'representations': [],
                'description': description,
                'rownum': '?',
                'url': None,
            }
        if course is not None:
            if self.course is not None:
                assert(self.course == Course(course))
            else:
                self.__p[name]['course'] = Course(course)
                self.course.activities.append(self)
        for p in [Concept(p) for p in prereqs if p is not '']:
            self.prereqs.append(p)
        for p in [Concept(p) for p in concepts if p is not '']:
            p.activities.append(self)
            if p not in self.concepts:
                self.concepts.append(p)
        if description is not None:
            self.__p[name]['description'] = description
        for r in representations:
            self.representations.append(r)
            if self not in r.activities:
              r.activities.append(self)
        if rownum is not None:
            self.__p[name]['rownum'] = rownum
        if url is not None:
            self.__p[name]['url'] = url
        if figure is not None:
            self.__p[name]['figure'] = figure
    def __eq__(self, other):
        return other is not None and self.name == other.name
    def __ne__(self, other):
        return other is None or self.name != other.name
    def __hash__(self):
        return hash(self.name)
    def __repr__(self):
        return 'Activity(%s)' % self.name
    @property
    def urlname(self):
        return slugify.slugify(self.name)
    @property
    def course(self):
        return self.__p[self.name]['course']
    @property
    def prereqs(self):
        return self.__p[self.name]['prereqs']
    @property
    def concepts(self):
        return self.__p[self.name]['concepts']
    @property
    def representations(self):
        return self.__p[self.name]['representations']
    @property
    def description(self):
        return self.__p[self.name]['description']
    @property
    def rownum(self):
        return self.__p[self.name]['rownum']
    @property
    def url(self):
        return self.__p[self.name]['url']
    @property
    def figure(self):
        return self.__p[self.name]['figure']

class Concept:
    """ A concept """
    __p = {}
    def __init__(self, name, course=None, prereqs=[], description=None, representations=[],
                 rownum=None, url=None, figure=None):
        while name[0] == ' ':
            name = name[1:]
        self.name = name
        if name not in self.__p:
            self.__p[name] = {
                'name': name,
                'course': None,
                'activities': [],
                'prereqs': [],
                'representations': [],
                'description': description,
                'rownum': '?',
                'url': None,
            }
        if course is not None:
            if self.course is not None:
                print('error', '"%s"' % self.course, '"%s"' % course, name, '"%s"' % name)
                print('"%s"' % Course(course))
                assert(self.course == Course(course))
            else:
                self.__p[name]['course'] = Course(course)
                if self not in self.course.concepts:
                    self.course.concepts.append(self)
        for r in representations:
            self.representations.append(r)
            if self not in r.concepts:
              r.concepts.append(self)
        for p in [Concept(p) for p in prereqs if p is not '']:
            self.prereqs.append(p)
        if description is not None:
            self.__p[name]['description'] = description
        if rownum is not None:
            self.__p[name]['rownum'] = rownum
        if url is not None:
            self.__p[name]['url'] = url
        if figure is not None:
            self.__p[name]['figure'] = figure
    def __eq__(self, other):
        return other is not None and self.name == other.name
    def __ne__(self, other):
        return other is None or self.name != other.name
    def __hash__(self):
        return hash(self.name)
    def __repr__(self):
        return 'Concept(%s)' % self.name
    @property
    def urlname(self):
        return slugify.slugify(self.name)
    @property
    def course(self):
        return self.__p[self.name]['course']
    @property
    def prereqs(self):
        return self.__p[self.name]['prereqs']
    @property
    def description(self):
        return self.__p[self.name]['description']
    @property
    def url(self):
        return self.__p[self.name]['url']
    @property
    def figure(self):
        return self.__p[self.name]['figure']
    @property
    def rownum(self):
        return self.__p[self.name]['rownum']
    @property
    def activities(self):
        return self.__p[self.name]['activities']
    @property
    def representations(self):
        return self.__p[self.name]['representations']

all_courses = [Course('MTH 251', 'Differential Calculus'),
               Course('MTH 254', 'Multivariable Calculus'),
               Course('PH 423', 'Energy and Entropy'),
               Course('MTH 255', 'Vector Calculus'),
               Course('PH 422', 'Static Fields'),
]

def parse_list(s):
    if len(s) == 0:
        return []
    if s[0] == '[' and s[-1] == ']':
        return list(filter(lambda x: x not in [''], s[1:-1].split(',')))
    else:
        return []

all_representations = []

class Representation:
    """ A representation """
    __p = {}
    def __init__(self, name, figure = None):
        while name[0] == ' ':
            name = name[1:]
        names = {
          'partial f/partial x': r'$\frac{\partial f}{\partial x}$',
          'partial f/partial x fixing y': r'$\left(\frac{\partial f}{\partial x}\right)_y$',
          'Del f': r'$\vec\nabla f$',
          'Del dot f': r'$\vec\nabla\cdot\vec f$',
          'df': r'$df$',
          'picture of PDM': 'PDM',
          'extable.jpg': 'Table',
        }
        if name in names:
          name = names[name]
        icons = {
          'partial f/partial x': r'$\frac{\partial f}{\partial x}$',
          'partial f/partial x fixing y': r'$\left(\frac{\partial f}{\partial x}\right)_y$',
          'Del f': r'$\vec\nabla f$',
          'Del dot f': r'$\vec\nabla\cdot\vec f$',
          'df': r'$df$',
          'Contour Maps': r'<img src="contour-map.svg"/>',
          'PDM': r'<img src="pdm.jpg"/>',
          'picture of PDM': r'<img src="pdm.jpg"/>',
          'Inclinometer': r'<img src="inclinometer.jpg"/>',
          'Kinesthetic': r'<img src="kin.jpg"/>',
          'Vector Field Map': r'<img src="vector-field-map.jpg"/>',
          '3D plots': r'<img src="3dplot.jpg"/>',
          'Table': r'$\begin{array}{c|c}x&y\\\hline3&0.2\\4&0.6\\5&0.9\end{array}$',
        }
        icon = name
        if name in icons:
          icon = icons[name]

        self.name = name
        if name not in self.__p:
            self.__p[name] = {
              'name': name,
              'icon': icon,
              'activities': [],
              'concepts': [],
              'description': 'NEED DESCRIPTION',
            }
        try:
          with open('descriptions/representation-{}.md'.format(self.urlname),'r') as f:
            self.__p[name]['description'] = markdown.markdown(f.read())
        except:
          print('unable to open', 'descriptions/representation-{}.md'.format(self.urlname))
          pass

        if figure is not None:
            self.__p[name]['figure'] = figure
        if self not in all_representations:
          all_representations.append(self)
    def __eq__(self, other):
        return other is not None and self.name == other.name
    def __ne__(self, other):
        return other is None or self.name != other.name
    def __hash__(self):
        return hash(self.name)
    def __repr__(self):
        return 'Representation(%s)' % self.name
    @property
    def icon(self):
        return self.__p[self.name]['icon']
    @property
    def urlname(self):
        return slugify.slugify(self.name)
    @property
    def description(self):
      return self.__p[self.name]['description']
    @property
    def url(self):
      return self.__p[self.name]['url']
    @property
    def figure(self):
      return self.__p[self.name]['figure']
    @property
    def activities(self):
      return self.__p[self.name]['activities']
    @property
    def concepts(self):
      return self.__p[self.name]['concepts']

with open('progression.csv', 'r') as csvfile:
     lines = list(csv.reader(csvfile, delimiter=',', quotechar='"'))
     for line in lines:
         kind = line[0]
         name = line[1]
         rownum = line[2]
         prereqs = parse_list(line[3])
         new_concepts = parse_list(line[4])
         representations = [Representation(r) for r in parse_list(line[5])]
         if len(representations) > 0:
             print('representations:', representations)
         course_number = line[6]
         figure = line[7]
         description = line[8]
         external_url = line[9]
         if ':' not in external_url and len(external_url) > 0:
             external_url = "http://physics.oregonstate.edu/portfolioswiki/acts:{}".format(external_url)
         status = line[10]
         if status == 'Active' and name != '':
             if kind == 'Concept':
                 #print('concept:', name, urlname)
                 concepts.append(Concept(name, course_number, prereqs,
                                         description=description,
                                         rownum=rownum,
                                         url=external_url,
                                         figure=figure,
                                         representations=representations))
             elif kind == 'Activity':
                 print('activity:', name, course_number)
                 activities.append(Activity(name, course_number, prereqs, new_concepts,
                                            rownum=rownum,
                                            description=description,
                                            url=external_url,
                                            figure=figure,
                                            representations=representations))

os.makedirs('output', exist_ok=True)

for course in all_courses:
    print('COURSE', course)
    name = course.name
    number = course.number
    prereq_courses = {}
    a = []
    concepts_in_activities = set()
    for x in activities:
        if x.course == course:
            a.append(x)
            concepts_in_activities.update(x.concepts)
    for x in a:
        for p in x.prereqs:
            if p.course != course and p.course is not None:
                if p.course not in prereq_courses:
                    prereq_courses[p.course] = set()
                prereq_courses[p.course].add(p)
    prereq_list = []
    for c in all_courses:
        if c in prereq_courses:
            these_concepts = [x for x in concepts if x in prereq_courses[c]]
            prereq_list.append((c, these_concepts))
    course_concepts = [x for x in concepts if x.course == course]
    course.orphan_concepts.extend([x for x in course_concepts if x not in concepts_in_activities])
    for x in course.orphan_concepts:
        print('  orphan:', x)
    with open('output/%s.html' % number, 'w') as f:
        f.write(course_template.render(course={
            'name': name,
            'number': number,
            'activities': a,
            'concepts': course_concepts,
            'orphan_concepts': course.orphan_concepts,
            'prereq_courses': prereq_list,
        }, style_css=style_css))

for activity in activities:
    with open('output/activity-%s.html' % activity.urlname, 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity,
                                                         style_css=style_css))

for concept in concepts:
    prereq_courses = {}
    prereq_course_hints = {}
    for p in concept.prereqs:
        if p.course != concept.course and p.course is not None:
            if p.course not in prereq_courses:
                prereq_courses[p.course] = set()
            prereq_courses[p.course].add(p)
    for a in concept.activities:
        for p in a.prereqs:
            if p.course != concept.course and p.course is not None:
                if p.course not in prereq_course_hints:
                    prereq_course_hints[p.course] = set()
                prereq_course_hints[p.course].add(p)
    prereq_list = []
    for c in all_courses:
        if c in prereq_courses or c in prereq_course_hints:
            these_concepts = []
            if c in prereq_courses:
                these_concepts = [x for x in concepts if x in prereq_courses[c]]
            hints = []
            if c in prereq_course_hints:
                hints = [x for x in concepts if x in prereq_course_hints[c] and x not in these_concepts]
            prereq_list.append((c, these_concepts, hints))
    concept.prereq_courses = prereq_list

    prereq_groups = []
    concept_activity_prereqs = set([c for a in concept.activities for c in a.prereqs])
    concept_activity_prereqs = concept_activity_prereqs.difference(concept.prereqs)
    for a in activities:
        ps = [c for c in a.concepts if c in concept.prereqs]
        hints = [c for c in a.concepts if c in concept_activity_prereqs]
        if len(ps) > 0 or len(hints) > 0:
            prereq_groups.append((a, ps, hints))
    concept.prereq_groups = prereq_groups

    output_concepts = list(map(lambda c: c.urlname, filter(lambda c: concept in c.prereqs, concepts)))
    output_groups = []
    for a in activities:
        ps = list(filter(lambda c: c.urlname in output_concepts, a.concepts))
        hints = list(filter(lambda c: c.urlname not in output_concepts, a.concepts))
        if len(ps) > 0:
            output_groups.append((a, ps, hints))
        elif concept in a.prereqs:
            output_groups.append((a,[], hints))
    concept.output_groups = output_groups
    with open('output/concept-%s.html' % concept.urlname, 'w') as f:
        f.write(env.get_template('concept.html').render(concept=concept,
                                                        style_css=style_css))

for activity in activities:
    prereq_courses = {}
    for p in activity.prereqs:
        if p.course is not None and p.course != activity.course:
            if p.course not in prereq_courses:
                prereq_courses[p.course] = set()
            prereq_courses[p.course].add(p)
    prereq_list = []
    for c in all_courses:
        if c in prereq_courses:
            these_concepts = [x for x in concepts if x in prereq_courses[c]]
            prereq_list.append((c, these_concepts))
    activity.prereq_courses = prereq_list

    prereq_groups = []
    for a in filter(lambda a: a.course == activity.course, activities):
        ps = [c for c in a.concepts if c in activity.prereqs]
        if len(ps) > 0:
            prereq_groups.append((a, ps))
    activity.prereq_groups = prereq_groups

    with open('output/activity-%s.html' % activity.urlname, 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity,
                                                         style_css=style_css))

with open('output/index.html', 'w') as f:
    f.write(env.get_template('progression.html').render(
        all_courses = all_courses,
        style_css=style_css,
        courses = [c for c in all_courses if len(c.activities) > 0],
        prereq_courses = [c for c in all_courses if len(c.activities) == 0],
    ))

for key in glob.glob('templates/*key.html'):
    print(key)
    key = key[len('templates/'):]
    with open('output/'+key, 'w') as f:
        f.write(env.get_template(key).render(style_css=style_css))

print('all descriptions:', glob.glob('descriptions/representation-*.md'))
for r in all_representations:
    other_concepts = copy.copy(r.concepts)
    groups = []
    for a in r.activities:
        ps = list(filter(lambda c: r in c.representations, a.concepts))
        for x in ps:
          other_concepts.remove(x)
        hints = []
        if len(ps) > 0 or r in a.representations:
            groups.append((a, ps, hints))
    r.groups = groups
    r.other_concepts = other_concepts
    with open('output/representation-%s.html' % r.urlname, 'w') as f:
        f.write(env.get_template('representation.html').render(representation=r,
                                                               style_css=style_css))
