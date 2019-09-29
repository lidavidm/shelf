{% for item in items %}
### _{{ item.name.alternatives[item.name.default] }}_ ({{ format_kind(kind=item.kind) }})
{% if item.people -%}
{% for person in item.people -%}
{% set p = lookup(collection=people, key=person[1]) -%}
- **{{ person[0] }}:** {{ p.name.alternatives[p.name.default] }}
{% endfor -%}
{% endif -%}
{% if item.rating -%}
- **Rating:** {{ item.rating }}/10
{% endif -%}
{% if item.completed -%}
- **Completed:** {{ item.completed }}
{% endif -%}

{% endfor %}
