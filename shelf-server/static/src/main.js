import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Items from "./Items.vue";

import EditAlternative from "./views/EditAlternative";
import EditPeople from "./views/EditPeople";
import EditSeries from "./views/EditSeries";
import EditTags from "./views/EditTags";

Vue.use(VueRouter);
Vue.component("edit-alternative", EditAlternative);
Vue.component("edit-people", EditPeople);
Vue.component("edit-series", EditSeries);
Vue.component("edit-tags", EditTags);

const router = new VueRouter({
    routes: [
        {
            path: "/",
            redirect: "items"
        },
        {
            path: "/items",
            component: Items,
        },
    ],
});

new Vue({
    el: "#app",
    router,
    render: h => h(App)
});
