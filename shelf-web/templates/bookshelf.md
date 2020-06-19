{% for item in items %}
{% if item.status == "InProgress" -%}
### (In Progress) _{{ item.name.alternatives[item.name.default] }}_ ({{ format_kind(kind=item.kind) }})
{% if item.people -%}
{% for person in item.people -%}
{% set p = lookup(collection=people, key=person[1]) -%}
- **{{ person[0] }}:** {{ p.name.alternatives[p.name.default] }}
{% endfor -%}
{% endif -%}
{% if item.tags -%}
- **Tags:** {% for tag in item.tags %}“{{tag}}” {% endfor %}
{% endif -%}
{% if item.extra and item.extra.external_url -%}
- **Link:** [{{item.extra.external_url}}]({{item.extra.external_url}})
{% endif -%}
{% endif -%}

{% endfor %}

---

{% for item in items %}
{% if item.status == "Completed" -%}
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
{% if item.tags -%}
- **Tags:** {% for tag in item.tags %}“{{tag}}” {% endfor %}
{% endif -%}
{% if item.extra and item.extra.external_url -%}
- **Link:** [{{item.extra.external_url}}]({{item.extra.external_url}})
{% endif -%}
{% endif -%}

{% endfor %}
