import { createApp } from "vue";
import App from "./App.vue";
// 额外引入图标库.Arco图标是一个独立的库，需要额外引入并注册使用。
import ArcoVueIcon from '@arco-design/web-vue/es/icon';
import '@arco-design/web-vue/dist/arco.less';
import router from "./router";

const app = createApp(App);
app.use(ArcoVueIcon);
app.use(router);
app.mount('#app');