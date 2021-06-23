<template lang="pug">
.co-colorpicker
  .color-type
    coDropdown
  .color-viewport
  .current-color
  // Color code
  // Color component sliders
  // Color component inputs
</template>

<script lang="ts">
  import { defineComponent } from 'vue';
  import coDropdown from '../co-dropdown.vue';

  export default defineComponent({
    name: 'coColorPicker',
    components: {
      coDropdown
    }
  });
</script>

<style lang="scss" scoped>
.co-colorpicker {
  display: grid;
  width: 100%;
  height: 40rem;
  background-color: #ddd;
}
</style>