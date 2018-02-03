from jinja2 import Environment, FileSystemLoader
import os

env = Environment(
    loader=FileSystemLoader('templates'),
)

course_template = env.get_template('course.html')

c = { 'name': 'PH441',
}

os.makedirs('output', exist_ok=True)

with open('output/ph441.html', 'w') as f:
    f.write(course_template.render(course=c))
