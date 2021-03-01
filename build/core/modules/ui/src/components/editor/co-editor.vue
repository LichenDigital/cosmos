<template lang="pug">
.co-editor
  coComponents
  coWorkspace
  coStyle
</template>

<script lang="ts">
import coComponents from './co-components.vue';
import coWorkspace from './co-workspace.vue';
import coStyle from './co-style.vue';

export default {
  name: 'editor',
  components: {
    coComponents,
    coWorkspace,
    coStyle
  }
}
</script>

<style lang="scss" scoped>
  .co-editor {
    display: flex;
    min-height: 100vh;
    width: 100%;
  }

</style>