from jinja2 import Environment, FileSystemLoader

env = Environment(
    loader=FileSystemLoader('templates'),
)

course_template = env.get_template('course.html')

c = { 'name': 'PH441',
}

print course_template.render(course=c)
