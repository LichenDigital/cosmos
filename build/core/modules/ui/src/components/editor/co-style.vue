<template lang="pug">
.co-style
  .size hey
  .position
  .effects
  .typography
  .spacing
  .layout
  .backgrounds
  .borders
</template>

<script lang="ts">
import CoInput from '../co-input.vue';

export default {
  name: 'co-style',
  components: {
    'co-input': CoInput
  }
}
</script>


<style lang="scss" scoped>
.co-style {
  width: 40%;
  height: 100%;
  display: block;
  background: blue;
}

</style>
