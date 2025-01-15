import {createApp} from 'vue'
import App from './App.vue'
import router from './router'
import pinia from './store';

// Vuetify配置
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import {createVuetify} from 'vuetify'
import {md3} from 'vuetify/blueprints'
import {aliases, mdi} from 'vuetify/iconsets/mdi-svg'
// Vue Toastification配置
import Toast, {PluginOptions, POSITION, TYPE} from "vue-toastification";
import "vue-toastification/dist/index.css";

const vuetify = createVuetify({
    blueprint: md3,
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi,
        },
    },
})

const options: PluginOptions = {
    toastDefaults: {
        [TYPE.SUCCESS]: {
            timeout: 1500,
            position: POSITION.TOP_CENTER,
            hideProgressBar: true,
        },
        [TYPE.ERROR]: {
            timeout: 2000,
            position: POSITION.TOP_RIGHT,
            closeButton: false,
        },
        [TYPE.INFO]: {
            timeout: 2000,
            position: POSITION.BOTTOM_CENTER,
            closeButton: false,
        },
        [TYPE.WARNING]: {
            timeout: 2000,
            position: POSITION.TOP_RIGHT,
            closeButton: false,
        },
    }
};

let app = createApp(App);
app.use(pinia);
app.use(router);
app.use(vuetify);
app.use(Toast, options);
app.mount('#app');
