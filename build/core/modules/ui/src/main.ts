import { createApp } from 'vue';
import App from './App.vue';
import './registerServiceWorker';
import { createStore } from 'vuex';

const store = createStore({
  state () {
    return {
      count: 0,
    }
  },
  mutations: {
    increment (state: any) {
      state.count++;
    }
  }
});


const app = createApp(App);
app.use(store);
app.mount('#app');