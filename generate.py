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
    c['course'] = course
    c['prereqs'] = fix_concept_list(prereqs)
    c['concepts'] = fix_concept_list(concepts)
    for x in c['concepts']:
        x['activity'] = c;
    activities.append(c)
    activity_map[urlname] = c
    activity_map[name] = c
    return c
def new_concept(urlname, name, prereqs):
    if urlname in concept_map:
        c = concept_map[urlname]
    elif name in  concept_map:
        c = concept_map[name]
    else:
        c = {}
    c['name'] = name
    c['urlname'] =  urlname
    c['course'] = None
    c['prereqs'] = fix_concept_list(prereqs)
    concepts.append(c)
    concept_map[urlname] = c
    concept_map[name] = c
    return c

new_concept('eating', 'eating', ['food'])
new_activity('activity-1', 'activity 1', 'PH423', ['writing', 'reading'], ['eating'])
new_activity('activity-2', 'activity 2', 'PH423', ['arithmetic', 'eating'], ['sewing'])

os.makedirs('output', exist_ok=True)

def create_course(number, name):
    a = []
    for x in activities:
        if x['course'] in [name,number]:
            a.append(x)
    c = []
    for x in concepts:
        if x['course'] in [name,number]:
            c.append(x)
    with open('output/%s.html' % number, 'w') as f:
        f.write(course_template.render(course={
            'name': name,
            'number': number,
            'activities': a,
            'concepts': c,
        }))

create_course('PH441', 'Capstone: Statistical Mechanics')
create_course('PH423', 'Energy and Entropy')
