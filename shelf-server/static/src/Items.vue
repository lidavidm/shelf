<template>
    <section id="items">

        <label for="import">Import MAL:</label>
        <input id="import" type="file" v-on:change="onFile" />

        Stats: {{ items.length }} items

        <table>
            <thead>
                <tr>
                    <th>Type</th>
                    <th>Name</th>
                    <th>Status</th>
                    <th>Entries</th>
                    <th>Rating</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="item in items">
                    <td>{{ item["kind"] }}</td>
                    <td>
                        {{ item["name"]["alternatives"][item["name"]["default"]] }}
                        <button
                            class="edit"
                            v-on:click="edit(item['key'])"
                        >
                            Edit
                        </button>
                    </td>
                    <td>{{ item["status"] }}</td>
                    <td>{{ item["entries"].filter(e => e.completed).length }} / {{ item["entries"].length }}</td>
                    <td>{{ item["rating"] === null ? "-" : item["rating"] }}</td>
                </tr>
            </tbody>
        </table>

        <EditItem v-if="editing" v-bind:item="editing" v-on:done="doneEditing" />
    </section>
</template>

<script>
    import firstBy from "thenby";
    import EditItem from "./EditItem";

    export default {
        name: "items",
        data() {
            return {
                items: [],
                editing: null,
            };
        },
        mounted() {
            window.fetch("/item")
                  .then(r => r.json())
                  .then((items) => {
                      this.items = items;
                      this.sortItems();
                  });
        },
        methods: {
            sortItems() {
                return this.items.sort(firstBy(v => v.kind)
                    .thenBy(v => v.status)
                    .thenBy(v => v.name.alternatives[v.name.default].toLowerCase()));
            },

            edit(key) {
                this.editing = key;
            },

            doneEditing() {
                this.editing = null;
            },

            onFile(e) {
                const files = e.target.files;
                if (files.length === 0) return;

                const reader = new FileReader();
                reader.onload = () => {
                    const parser = new DOMParser();
                    const doc = parser.parseFromString(reader.result, "application/xml");
                    console.log(doc);

                    const list = doc.documentElement;
                    for (const child of list.children) {
                        if (child.nodeName === "anime" || child.nodeName === "manga") {
                            const titleField = child.nodeName === "manga" ?
                                               "manga_title" : "series_title";
                            const episodesField = child.nodeName === "manga" ?
                                                  "manga_chapters" : "series_episodes";
                            const watchedField = child.nodeName === "manga" ?
                                                 "my_read_chapters" : "my_watched_episodes";

                            const title = child.querySelector(titleField).textContent.trim();
                            const key = title
                                .replace(/[\W:]+/g, "-")
                                .replace(/^-+/, "")
                                .replace(/-+$/, "")
                                .toLowerCase();
                            let kind = "Unknown";
                            if (child.nodeName === "manga") {
                                kind = "Manga";
                            }
                            else {
                                switch (child.querySelector("series_type").textContent.trim()) {
                                    case "TV":
                                    case "Special":
                                        kind = "TV";
                                        break;
                                    case "Movie":
                                        kind = "Film";
                                        break;
                                    case "OVA":
                                        kind = "OVA";
                                        break;
                                    case "ONA":
                                        kind = "ONA";
                                        break;
                                    case "Music":
                                        kind = "Music";
                                        break;
                                }
                            }
                            const score = parseInt(child.querySelector("my_score").textContent.trim(), 10);

                            let status = "Planned";
                            switch (child.querySelector("my_status").textContent.trim()) {
                                case "Plan to Watch":
                                case "Plan to Read":
                                    status = "Planned";
                                    break;
                                case "Completed":
                                    status = "Completed";
                                    break;
                                case "Dropped":
                                    status = "Dropped";
                                    break;
                                case "On-Hold":
                                    status = "OnHold";
                                    break;
                                case "Watching":
                                case "Reading":
                                    status = "InProgress";
                                    break;
                            }

                            const result = {
                                key,
                                kind,
                                name: {
                                    default: "Japanese (Romaji)",
                                    alternatives: {
                                        "Japanese (Romaji)": title,
                                    },
                                },
                                people: [],
                                season: null,
                                entries: [],
                                status,
                                rating: (score === 0 || !Number.isFinite(score)) ? null : score,
                                added: new Date().toISOString(),
                                started: child.querySelector("my_start_date").textContent.trim(),
                                completed: child.querySelector("my_finish_date").textContent.trim(),
                            };

                            const episodes = parseInt(child.querySelector(episodesField).textContent.trim(), 10);
                            const watched = parseInt(child.querySelector(watchedField).textContent.trim(), 10);

                            for (let i = 0; i < Math.max(episodes, watched); i++) {
                                result.entries.push({
                                    name: null,
                                    number: i + 1,
                                    volume: null,
                                    completed: i < watched,
                                });
                            }

                            window.fetch("/item", {
                                method: "PUT",
                                body: JSON.stringify(result),
                                headers: {
                                    "Content-Type": "application/json",
                                },
                            });

                            this.items.push(result);
                        }
                    }
                    this.sortItems();
                };
                reader.readAsText(files[0]);
            },
        },
        components: {
            EditItem,
        },
    };
</script>

<style lang="css">
    table {
        border-collapse: collapse;
        min-width: 60em;
    }

    th {
        border-bottom: 1px solid black;
        text-align: left;
    }

    .edit {
        float: right;
    }
</style>
