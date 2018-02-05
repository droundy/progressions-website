from jinja2 import Environment, FileSystemLoader
import os

env = Environment(
    loader=FileSystemLoader('templates'),
)

course_template = env.get_template('course.html')

activities = []
concepts = []

activity_map = {}
concept_map = {}

all_courses = [
    {'number': 'winco', 'name': 'grocery store'},
    {'number': 'K12', 'name': 'elementary school'},
    {'number': 'PH423', 'name': 'Energy and Entropy'},
    {'number': 'PH441', 'name': 'Thermal Capstone'},
]
course_map = {}
for c in all_courses:
    course_map[c['name']] = c
    course_map[c['number']] = c
    c['activities'] = []
    c['concepts'] = []

def lookup_activity(a):
    if a not in activity_map:
        new_activity(a,a,'???', [], [])
    return activity_map[a]
def lookup_concept(c):
    if c not in concept_map:
        new_concept(c,c,[])
    return concept_map[c]
def fix_concept_list(xs):
    output = []
    for x in xs:
        output.append(lookup_concept(x))
    return output

def new_activity(urlname, name, course, prereqs, concepts):
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

new_concept('reading', 'reading', [], 'elementary school')
new_concept('writing', 'writing', [], 'elementary school')
new_concept('arithmetic', 'arithmetic', [], 'elementary school')
new_concept('food', 'food', [], 'grocery store')

new_concept('tailoring', 'tailoring', ['sewing'])

new_concept('eating', 'eating', ['food'])

new_activity('activity-1', 'activity 1', 'PH423', ['writing', 'reading'], ['eating'])
new_activity('activity-2', 'activity 2', 'PH423', ['food', 'arithmetic', 'eating'], ['sewing'])
new_activity('activity-0', 'activity 0 (last)', 'PH423', ['sewing'],
             ['fitting', 'tailoring'])

new_activity('senior-1', 'senior activity', 'PH441', ['tailoring'],
             ['fashion'])

os.makedirs('output/activity', exist_ok=True)
os.makedirs('output/concept', exist_ok=True)

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
            if p['course'] != course:
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
    with open('output/activity/%s.html' % activity['urlname'], 'w') as f:
        f.write(env.get_template('activity.html').render(activity=activity))

for concept in concepts:
    prereq_courses = {}
    for p in concept['prereqs']:
        if p['course'] != concept['course']:
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
    concept['output_groups'] = output_groups
    with open('output/concept/%s.html' % concept['urlname'], 'w') as f:
        f.write(env.get_template('concept.html').render(concept=concept))

with open('output/index.html', 'w') as f:
    f.write(env.get_template('progression.html').render(
        all_courses = all_courses,
        courses = [c for c in all_courses if len(c['activities']) > 0],
        prereq_courses = [c for c in all_courses if len(c['activities']) == 0],
    ))
