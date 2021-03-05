<template lang="pug">
.co-components.co-group
  h2.co-panel-label components
  h3 button
  coButton(:link='{destination: "https://lichen.co"}') button
  h3 textInput
  h3 progress
  coProgress
  h3 link
  h3 text


</template>

<script lang="ts">

import coButton from '../co-button.vue';
import coProgress from '../co-progress.vue';

export default {
  name: 'components',
  components: {
    coButton,
    coProgress
  }
}
</script>

<style lang="scss" scoped>
.co-components {
  width: 35%;
  background-color: #e1e1e1;
}
</style>
