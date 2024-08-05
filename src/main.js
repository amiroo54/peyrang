import { createApp } from "vue";
import App from "./App.vue";
import Vue3ColorPicker from "vue3-colorpicker";
import "vue3-colorpicker/style.css";


// Router
import { createMemoryHistory, createRouter } from 'vue-router'

import MainView from "./views/MainView.vue";
import ReplaceView from "./views/ReplaceView.vue";
import PermutateView from "./views/PermutateView.vue";
import OKLabSetView from "./views/OKLabSetView.vue";
import OKLabShiftView from "./views/OKLabShiftView.vue";

import SettingsView from "./views/SettingsView.vue";

const routes = [
    { path: '/', component: MainView},
    { path: '/replace', component: ReplaceView},
    { path: '/permutate', component: PermutateView},
    { path: '/set', component: OKLabSetView},
    { path: '/shift', component: OKLabShiftView},
    { path: '/settings', component: SettingsView},
]

const router = createRouter({history: createMemoryHistory(), routes});

createApp(App).use(Vue3ColorPicker).use(router).mount("#app");
