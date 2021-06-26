<template lang="pug">
component(:is='type' v-if='validators.type(type)') {{ content }}
</template>

<script lang="ts">
import { defineComponent } from 'vue';

let validators = {
  type(value: string) { return['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(value) },
};

export default defineComponent({
  name: 'coHeading',
  props: {
    type: {
      default: 'h1',
      type: String,
      validator(value: string) {
        return validators.type(value);
      }
    },
    content: {
      default: '',
      type: String
    }
  },
  data() {
    return {
      validators: validators
    };
  }
});
</script>

<style lang="scss" scoped>
  
</style>