<template lang="pug">
.co-colorpicker
  .color-type
    coDropdown
  .color-map
    .color-saturation
    .color-brightness
  .color-current(:style='{backgroundColor: colorCurrent}')
  .color-code
    coTextInput
  .color-sliders
    .color-slider(v-for='value, index in [1,2,3,4]' :key='index')
      coSlider
      coTextInput
</template>

<script lang="ts">
  import { defineComponent } from 'vue';
  import coDropdown from '../co-dropdown.vue';
  import coSlider from '../co-slider.vue';
  import coTextInput from '../co-textinput.vue';

  export default defineComponent({
    name: 'coColorPicker',
    components: {
      coDropdown,
      coSlider,
      coTextInput
    },
    data() {
      return {
        colorCurrent: '#faa'
      };
    }
  });
</script>

<style lang="scss" scoped>
.co-colorpicker {
  display: grid;
  grid-template-columns: 2fr 1fr;
  grid-template-rows: auto 4fr auto;
  width: 100%;
  height: 40rem;
  background-color: #ddd;
}
.color {
  &-type {
    grid-column: 1 / 3;
  }
  &-map {
    grid-column: 1 / 2;
    grid-row: 2 / 3;
    background: red;
    position: relative;
  }
  &-saturation {
    position: absolute;
    top: 0; left: 0;
    width: 100%; height: 100%;
    background: linear-gradient(90deg, #fff, hsla(0,0%,100%,0));
  }
  &-brightness {
    position: absolute;
    top: 0; left: 0;
    width: 100%; height: 100%;
    background: linear-gradient(hsla(0,0%,0%,0), #000);
  }
  &-sliders {
    grid-column: 2 / 3;
    grid-row: 2 / 3;
  }
  &-current {
    grid-column: 1 / 2;
    grid-row: 3 / 4;
  }
  &-color-code {
    grid-column: 2 / 3;
    grid-row: 3 / 4;
  }
}

</style>