<template>
    <table>
        <thead>
            <tr>
                <th>Person</th>
                <th>Role</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="(person, idx) in people" v-bind:key="idx">
                <td>
                    <select v-model="person[1]" v-on:change="update">
                        <option
                            v-for="choice in allPeople"
                            v-bind:value="choice.key"
                        >
                            {{ choice.name.alternatives[choice.name.default]}}
                        </option>
                    </select>
                </td>
                <td>
                    <select v-model="person[0]" v-on:change="update">
                        <option>Author</option>
                        <option>Translator</option>
                        <option>Artist</option>
                    </select>
                </td>
            </tr>
            <tr>
                <td>
                    <button v-on:click="addPerson">Add Person</button>
                    <button v-on:click="createPerson">Create Person</button>
                </td>
                <td v-if="creatingPerson">
                    <label for="new-person-name">Name:</label>
                    <input id="new-person-name" type="text" v-model="newPersonName" />
                    <button v-on:click="finishPerson">Create</button>
                </td>
            </tr>
        </tbody>
    </table>
</template>

<script>
    import firstBy from "thenby";
    import * as util from "../util";

    export default {
        name: "edit-people",
        props: {
            value: Array,
        },
        data() {
            return {
                allPeople: [],
                people: this.value.slice(),
                creatingPerson: false,
                newPersonName: "",
            };
        },
        mounted() {
            this.getPeople();
        },
        methods: {
            getPeople() {
                this.allPeople = [];
                window.fetch("/person")
                      .then(r => r.json())
                      .then((people) => {
                          people.sort(firstBy(x => x.name.alternatives[x.name.default]));
                          for (const person of people) {
                              this.allPeople.push(person);
                          }
                      });
            },
            addPerson() {
                this.people.push(["Author", this.allPeople[0].key]);
                this.update();
            },
            createPerson() {
                this.creatingPerson = true;
            },
            finishPerson() {
                const name = this.newPersonName.trim();
                const key = util.makeKey("person", name);
                window.fetch(`/person`, {
                    method: "PUT",
                    body: JSON.stringify({
                        key,
                        name: {
                            default: "English",
                            alternatives: {
                                "English": name,
                            },
                        },
                    }),
                    headers: {
                        "Content-Type": "text/json",
                    },
                }).then(req => req.text()).then(() => this.getPeople());
                this.creatingPerson = false;
                this.newPersonName = "";
            },

            update() {
                this.$emit("input", this.people);
            },
        },
    };
</script>

<style lang="css">
</style>
