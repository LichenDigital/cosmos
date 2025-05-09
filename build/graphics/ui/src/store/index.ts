import { createStore } from 'vuex'

export default createStore({
  state: {
    log: {
      level: 'verbose',
      enabled: true,
      data: [] as string[]
    }
  },
  mutations: {
    addEntry(state, entry: string) {
      state.log.data.push(entry);
      console.log(entry);
    }
  },
  actions: {
  },
  modules: {
  }
})
