from jinja2 import Environment, FileSystemLoader
import os, csv, slugify, glob, copy

env = Environment(
    loader=FileSystemLoader('templates'),
)

course_template = env.get_template('course.html')

activities = []
concepts = []

class Course:
    """ A course """
    __p = {}
    def __init__(self, number, name=None):
        if number not in Course.__p:
            self.__p[number] = {'number': number, 'name': name, 'activities': [], 'concepts': []}
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
    @property
    def name(self):
        return self.__p[self.number]['name']
    @property
    def activities(self):
        return self.__p[self.number]['activities']
    @property
    def concepts(self):
        return self.__p[self.number]['concepts']

class Activity:
    """ A teaching activity """
    __p = {}
    def __init__(self, name, course=None, prereqs=[], concepts=[], representations=[],
                 description=''):
        self.name = name
        if name not in self.__p:
            self.__p[name] = {
                'name': name,
                'course': None,
                'prereqs': [],
                'concepts': [],
                'representations': [],
                'description': description,
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
            p.activity = self
            self.concepts.append(p)
        if description is not None:
            self.__p[name]['description'] = description
        for r in representations:
            while r[0] == ' ': # cut any leading whitespace
                r = r[1:]
            self.representations.append(r)
    def __eq__(self, other):
        return other is not None and self.name == other.name
    def __ne__(self, other):
        return other is None or self.name != other.name
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

class Concept:
    """ A concept """
    __p = {}
    def __init__(self, name, course=None, prereqs=[], description=None):
        while name[0] == ' ':
            name = name[1:]
        self.name = name
        if name not in self.__p:
            self.__p[name] = {
                'name': name,
                'course': None,
                'activity': None,
                'prereqs': [],
                'description': description,
            }
        if course is not None:
            if self.course is not None:
                print('error', '"%s"' % self.course, '"%s"' % course, name, '"%s"' % name)
                print('"%s"' % Course(course))
                assert(self.course == Course(course))
            else:
                self.__p[name]['course'] = Course(course)
                self.course.concepts.append(self)
        if course is not None:
            self.__p[name]['course'] = Course(course)
            self.course.concepts.append(self)
        for p in [Concept(p) for p in prereqs if p is not '']:
            self.prereqs.append(p)
        if description is not None:
            self.__p[name]['description'] = description
    def __eq__(self, other):
        return other is not None and self.name == other.name
    def __ne__(self, other):
        return other is None or self.name != other.name
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
    def activity(self):
        return self.__p[self.name]['activity']
    @activity.setter
    def activity(self, a):
        self.__p[self.name]['activity'] = a

all_courses = [Course('MTH 251', 'Differential Calculus'),
               Course('MTH 254', 'Multivariable Calculus'),
               Course('PH 423', 'Energy and Entropy'),
               Course('MTH 255', 'Vector Calculus'),
               Course('PH 422', 'Static Fields'),
]

def parse_list(s):
    if s[0] == '[' and s[-1] == ']':
        return list(filter(lambda x: x not in [''], s[1:-1].split(',')))
    else:
        return []

def clean_representation(r):
    reprs = {
        'partial f/partial x': r'$\frac{\partial f}{\partial x}$',
        'partial f/partial x fixing y': r'$\left(\frac{\partial f}{\partial x}\right)_y$',
        'Del f': r'$\vec\nabla f$',
        'Del dot f': r'$\vec\nabla\cdot\vec f$',
        'df': r'$df$',
        'Contour Maps': r'<img src="contour-map.svg"/>',
        'Inclinometer': r'<img src="inclinometer.jpg"/>',
    }
    if r in reprs:
        return reprs[r]
    if r[0] == ' ' and r[1:] in reprs:
        return reprs[r[1:]]
    print('not in reprs: "%s"' % r)
    return r

with open('progression.csv', 'r') as csvfile:
     lines = list(csv.reader(csvfile, delimiter=',', quotechar='"'))
     for line in lines:
         kind = line[0]
         name = line[1]
         urlname = line[2]
         if urlname == '':
             urlname = slugify.slugify(name)
         prereqs = parse_list(line[3])
         new_concepts = parse_list(line[4])
         representations = [clean_representation(r) for r in parse_list(line[5])]
         if len(representations) > 0:
             print('representations:', representations)
         course_number = line[6]
         generic_course = line[7]
         description = line[8]
         external_url = line[9]
         status = line[10]
         if status == 'Active' and name != '':
             if kind == 'Concept':
                 #print('concept:', name, urlname)
                 concepts.append(Concept(name, course_number, prereqs,
                                         description=description))
             elif kind == 'Activity':
                 print('activity:', name, course_number)
                 activities.append(Activity(name, course_number, prereqs, new_concepts,
                                            description=description,
                                            representations=representations))

# new_concept('reading', 'reading', [], 'elementary school')
# new_concept('writing', 'writing', [], 'elementary school')
# new_concept('arithmetic', 'arithmetic', [], 'elementary school')
# new_concept('food', 'food', [], 'grocery store')

# new_concept('tailoring', 'tailoring', ['sewing'])

# new_concept('eating', 'eating', ['food'])

# new_activity('activity-1', 'activity 1', 'PH 423', ['writing', 'reading'], ['eating'])
# new_activity('activity-2', 'activity 2', 'PH 423', ['food', 'arithmetic', 'eating'], ['sewing'])
# new_activity('activity-0', 'activity 0 (last)', 'PH 423', ['sewing'],
#              ['fitting', 'tailoring'])

# new_activity('senior-1', 'senior activity', 'PH 441', ['tailoring'],
#              ['fashion'])

os.makedirs('output', exist_ok=True)

for course in all_courses:
    print('COURSE', course)
    name = course.name
    number = course.number
    prereq_courses = {}
    a = []
    for x in activities:
        if x.course == course:
            a.append(x)
            print('including', x)
        else:
            print('    not including', x.course, '!=', course, x)
    c = []
    for x in concepts:
        if x.course == course:
            c.append(x)
    for x in a:
        for p in x.prereqs:
            if p.course != course and p.course is not None:
                if p.course.number not in prereq_courses:
                    prereq_courses[p.course.number] = []
                prereq_courses[p.course.number].append(p)
    prereq_list = []
    for c in all_courses:
        if c.number in prereq_courses:
            prereq_list.append((c, prereq_courses[c.number]))
    with open('output/%s.html' % number, 'w') as f:
        f.write(course_template.render(course={
            'name': name,
            'number': number,
            'activities': a,
            'concepts': c,
            'prereq_courses': prereq_list,
        }))

for activity in activities:
    with open('output/activity-%s.html' % activity.urlname, 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity))

for concept in concepts:
    prereq_courses = {}
    for p in concept.prereqs:
        if p.course != concept.course and p.course is not None:
            if p.course.number not in prereq_courses:
                prereq_courses[p.course.number] = []
                prereq_courses[p.course.number].append(p)
    prereq_list = []
    for c in all_courses:
        if c.number in prereq_courses:
            prereq_list.append((c, prereq_courses[c.number]))
    concept.prereq_courses = prereq_list

    prereq_groups = []
    for a in activities:
        ps = list(filter(lambda c: c in concept.prereqs, a.concepts))
        if len(ps) > 0:
            prereq_groups.append((a, ps))
    concept.prereq_groups = prereq_groups

    output_concepts = list(map(lambda c: c.urlname, filter(lambda c: concept in c.prereqs, concepts)))
    output_groups = []
    for a in activities:
        ps = list(filter(lambda c: c.urlname in output_concepts, a.concepts))
        if len(ps) > 0:
            output_groups.append((a, ps))
        elif concept in a.prereqs:
            output_groups.append((a,[]))
    concept.output_groups = output_groups
    with open('output/concept-%s.html' % concept.urlname, 'w') as f:
        f.write(env.get_template('concept.html').render(concept=concept))

for activity in activities:
    prereq_courses = {}
    for p in activity.prereqs:
        if p.course is not None and p.course.number != activity.course.number:
            if p.course.number not in prereq_courses:
                prereq_courses[p.course.number] = []
                prereq_courses[p.course.number].append(p)
    prereq_list = []
    for c in all_courses:
        if c.number in prereq_courses:
            prereq_list.append((c, prereq_courses[c.number]))
    activity.prereq_courses = prereq_list

    prereq_groups = []
    for a in filter(lambda a: a.course.number == activity.course.number, activities):
        ps = [c for c in a.concepts if c in activity.prereqs]
        if len(ps) > 0:
            prereq_groups.append((a, ps))
    activity.prereq_groups = prereq_groups

    with open('output/activity-%s.html' % activity.urlname, 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity))

with open('output/index.html', 'w') as f:
    f.write(env.get_template('progression.html').render(
        all_courses = all_courses,
        courses = [c for c in all_courses if len(c.activities) > 0],
        prereq_courses = [c for c in all_courses if len(c.activities) == 0],
    ))

for key in glob.glob('templates/*key.html'):
    print(key)
    key = key[len('templates/'):]
    with open('output/'+key, 'w') as f:
        f.write(env.get_template(key).render())

print('activities are', activities)
