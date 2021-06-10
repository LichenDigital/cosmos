import { createStore } from 'vuex'

export default createStore({
  state: {
    log: {
      level: 'verbose',
      enabled: true,
      data: ''
    }
  },
  mutations: {
    addEntry(state, entry: string) {
      state.log.data += entry;
    }
  },
  actions: {
  },
  modules: {
  }
})
