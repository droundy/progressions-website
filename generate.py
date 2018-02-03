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

def new_activity(urlname, name, course, prereqs, concepts):
    activities.append({
        'name': name,
        'urlname': urlname,
        'course': course,
        'prereqs': prereqs,
        'concepts': concepts,
    })
def new_concept(urlname, name, prereqs):
    concepts.append({
        'name': name,
        'urlname': urlname,
        'course': None,
        'prereqs': prereqs,
    })

new_concept('eating', 'eating', ['food']
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
