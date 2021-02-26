<template>
  <div id="app">
    <co-components>
      <co-button>Button</co-button>
      <co-input>Text</co-input>
    </co-components>
    <co-workspace></co-workspace>
  </div>
</template>

<script>
import coWorkspace from './components/workspace/co-workspace.vue';
import coComponents from './components/workspace/co-components.vue';
import coButton from './components/co-button.vue';
import coInput from './components/co-input.vue';

export default {
  name: 'app',
  components: {
    'co-workspace': coWorkspace,
    'co-components': coComponents,
    'co-button': coButton,
    'co-input': coInput
  }
}
</script>

<style>
#app {
  display: flex;
  justify-content: space-between;
  height: 100vh;
}
</style>
