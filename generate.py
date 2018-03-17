from jinja2 import Environment, FileSystemLoader
import os, csv, slugify, glob

env = Environment(
    loader=FileSystemLoader('templates'),
)

course_template = env.get_template('course.html')

activities = []
concepts = []

activity_map = {}
concept_map = {}

all_courses = [
    # {'number': 'winco', 'name': 'grocery store'},
    # {'number': 'K12', 'name': 'elementary school'},
    # {'number': 'PH 423', 'name': 'Energy and Entropy'},
    {'number': 'MTH 251', 'name': 'Differential Calculus'},
    {'number': 'MTH 254', 'name': 'Multivariable Calculus'},
    {'number': 'PH 422', 'name': 'Static Fields'},
    # {'number': 'PH 441', 'name': 'Thermal Capstone'},
]
course_map = {}
for c in all_courses:
    course_map[c['name']] = c
    course_map[c['number']] = c
    c['activities'] = []
    c['concepts'] = []

def lookup_activity(a):
    if a not in activity_map:
        new_activity(slugify.slugify(a),a,'???', [], [])
    return activity_map[a]
def lookup_concept(c):
    if c not in concept_map:
        new_concept(slugify.slugify(c),c,[])
    return concept_map[c]
def fix_concept_list(xs):
    output = []
    for x in xs:
        output.append(lookup_concept(x))
    return output

def new_activity(urlname, name, course, prereqs, concepts, representations=[]):
    if urlname in activity_map:
        c = activity_map[urlname]
    elif name in  activity_map:
        c = activity_map[name]
    else:
        c = {}
    c['name'] = name
    c['urlname'] =  urlname
    course = course_map[course]
    course['activities'].append(c)
    c['course'] = course
    c['representations'] = representations
    c['prereqs'] = fix_concept_list(prereqs)
    c['concepts'] = fix_concept_list(concepts)
    for x in c['concepts']:
        x['activity'] = c;
        if x['course'] is not None and x['course'] != c['course']:
            print('Inconsistency: activity {} and concept {} have inconsistent courses: {} vs. {}'.format(
                name, x['name'], course, x['course']
            ))
        x['course'] = c['course']
    activities.append(c)
    activity_map[urlname] = c
    activity_map[name] = c
    return c
def new_concept(urlname, name, prereqs, course=None):
    if urlname in concept_map:
        c = concept_map[urlname]
    elif name in  concept_map:
        c = concept_map[name]
    else:
        c = {}
    c['name'] = name
    c['urlname'] =  urlname
    c['course'] = None
    if course is not None:
        c['course'] = course_map[course]
        course_map[course]['concepts'].append(c)
    c['prereqs'] = fix_concept_list(prereqs)
    concepts.append(c)
    concept_map[urlname] = c
    concept_map[name] = c
    return c

def parse_list(s):
    if s[0] == '[' and s[-1] == ']':
        return list(filter(lambda x: x not in [''], s[1:-1].split(',')))
    else:
        return []

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
         representations = parse_list(line[5])
         if len(representations) > 0:
             print('representations:', representations)
         course_number = line[6]
         generic_course = line[7]
         description = line[8]
         external_url = line[9]
         status = line[10]
         if status == 'Active':
             if kind == 'Concept':
                 print('concept:', name, urlname)
                 new_concept(urlname, name, prereqs, course_number)
             elif kind == 'Activity':
                 print('activity:', name)
                 new_activity(urlname, name, course_number, prereqs, new_concepts,
                              representations=representations)

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
    name = course['name']
    number = course['number']
    prereq_courses = {}
    a = []
    for x in activities:
        if x['course'] == course:
            a.append(x)
    c = []
    for x in concepts:
        if x['course'] == course:
            c.append(x)
    for x in a:
        for p in x['prereqs']:
            if p['course'] != course and p['course'] is not None:
                if p['course']['number'] not in prereq_courses:
                    prereq_courses[p['course']['number']] = []
                prereq_courses[p['course']['number']].append(p)
    prereq_list = []
    for c in all_courses:
        if c['number'] in prereq_courses:
            prereq_list.append((c, prereq_courses[c['number']]))
    with open('output/%s.html' % number, 'w') as f:
        f.write(course_template.render(course={
            'name': name,
            'number': number,
            'activities': a,
            'concepts': c,
            'prereq_courses': prereq_list,
        }))

for activity in activities:
    with open('output/activity-%s.html' % activity['urlname'], 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity))

for concept in concepts:
    prereq_courses = {}
    for p in concept['prereqs']:
        if p['course'] != concept['course'] and p['course'] is not None:
            if p['course']['number'] not in prereq_courses:
                prereq_courses[p['course']['number']] = []
                prereq_courses[p['course']['number']].append(p)
    prereq_list = []
    for c in all_courses:
        if c['number'] in prereq_courses:
            prereq_list.append((c, prereq_courses[c['number']]))
    concept['prereq_courses'] = prereq_list

    prereq_groups = []
    for a in activities:
        ps = list(filter(lambda c: c in concept['prereqs'], a['concepts']))
        if len(ps) > 0:
            prereq_groups.append((a, ps))
    concept['prereq_groups'] = prereq_groups

    output_concepts = list(map(lambda c: c['urlname'], filter(lambda c: concept in c['prereqs'], concepts)))
    output_groups = []
    for a in activities:
        ps = list(filter(lambda c: c['urlname'] in output_concepts, a['concepts']))
        if len(ps) > 0:
            output_groups.append((a, ps))
        elif concept in a['prereqs']:
            output_groups.append((a,[]))
    concept['output_groups'] = output_groups
    with open('output/concept-%s.html' % concept['urlname'], 'w') as f:
        f.write(env.get_template('concept.html').render(concept=concept))

for activity in activities:
    prereq_courses = {}
    for p in activity['prereqs']:
        if p['course'] != activity['course'] and p['course'] is not None:
            if p['course']['number'] not in prereq_courses:
                prereq_courses[p['course']['number']] = []
                prereq_courses[p['course']['number']].append(p)
    prereq_list = []
    for c in all_courses:
        if c['number'] in prereq_courses:
            prereq_list.append((c, prereq_courses[c['number']]))
    activity['prereq_courses'] = prereq_list

    prereq_groups = []
    for a in activities:
        ps = list(filter(lambda c: c in activity['prereqs'], a['concepts']))
        if len(ps) > 0:
            prereq_groups.append((a, ps))
    activity['prereq_groups'] = prereq_groups

    with open('output/activity-%s.html' % activity['urlname'], 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity))

with open('output/index.html', 'w') as f:
    f.write(env.get_template('progression.html').render(
        all_courses = all_courses,
        courses = [c for c in all_courses if len(c['activities']) > 0],
        prereq_courses = [c for c in all_courses if len(c['activities']) == 0],
    ))

for key in glob.glob('templates/*key.html'):
    print(key)
    key = key[len('templates/'):]
    with open('output/'+key, 'w') as f:
        f.write(env.get_template(key).render())
