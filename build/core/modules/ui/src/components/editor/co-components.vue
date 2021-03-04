<template lang="pug">
.co-components.co-group
  .co-panel-label components
  coButton(:link='{destination: "https://lichen.co"}') test
</template>

<script lang="ts">

import coButton from '../co-button.vue';

export default {
  name: 'components',
  components: {
    coButton
  }
}
</script>

<style lang="scss" scoped>
.co-components {
  width: 35%;
  background-color: #e1e1e1;
}
</style>
