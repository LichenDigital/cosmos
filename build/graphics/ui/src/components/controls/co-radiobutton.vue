<template lang="pug">
div.co-radio-button
  input(type='radio' :name='group' :value='value')
  coIcon.co-pickmark(:icon='"check"')
label(v-if='label') {{ label }}
</template>

<script lang="ts">

import { defineComponent } from 'vue';
import coIcon from '../co-icon.vue';

export default defineComponent({
  name: 'coRadioButton',
  components: {
    coIcon
  },
  props: {
    group: {
      type: [String, Number],
      default: null
    },
    value: {
      type: [String, Number],
      default: null
    },
    label: {
      type: [String, Number],
      default: null
    }
  },
  data() {
    return {
      picked: null,
      changed: ''
    };
  },
  methods: {
    pickedToggle(element: any) {
      console.log(element.srcElement.value);
    }
  }
});
</script>

<style lang="scss" scoped>
  .co-radio-button {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    padding: 6px;

    input {
      border: 2px solid black;
      width: 100%;
      height: 100%;
      background: white;
      appearance: none;
      border-radius: 100%;
      outline: none;
      position: absolute;
      top: 0; left: 0;

      &:focus {
        box-shadow: 0 0 0 2px #888;
      }
      
      &:checked ~ .co-pickmark {
        opacity: 1;
      }
    }

    .co-pickmark {
      position: relative;
      opacity: 0;
      z-index: 1;
      pointer-events: none;
    }
  }
</style>