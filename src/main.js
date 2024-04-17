import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import {createRouter,createWebHashHistory} from 'vue-router'
import Nouns from "./pages/Nouns.vue";
import Verbs from "./pages/Verbs.vue";
import Propositions from "./pages/Propositions.vue";
import Phrases from "./pages/Phrases.vue";
import AddWord from "./pages/AddWord.vue";
import MedicalPhrases from "./pages/MedicalPhrases.vue";
import Verb from "./pages/Verb.vue";
import SearchPage from "./pages/SearchPage.vue";

const routes=[
    {path:"/noun",component:Nouns},
    {path:"/verb",component:Verbs},
    {path:"/proposition",component:Propositions},
    {path:"/phrase",component:Phrases},
    {path:"/add-word",component: AddWord},
    {path:"/medical-phrases",component: MedicalPhrases},
    {path:"/verbs/:id",component: Verb},
    {path:"/search/:query",component: SearchPage},
];

const router = createRouter({
    // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
})

createApp(App).use(router).mount('#app')
