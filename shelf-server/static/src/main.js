import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Items from "./Items.vue";

Vue.use(VueRouter);

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
