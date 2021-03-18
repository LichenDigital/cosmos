<template lang="pug">
//- Need to provide mechanisms for single line (input) or multiline (textarea) inputs
//- Need also to provide mechanisms for validation of different types of data (addresses, credit cards, emails etc.)
//- Also need to provide internal icon support.
//- Have predefined input, and also provide for complete customization

div(v-if='lines === 1')
  input(placeholder='input')

div(v-if='lines > 1')
  textarea(placeholder='textarea' :rows='lines')

</template>

<script lang="ts">

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'coTextInput',
  props: {
    'lines': {
      type: Number,
      default: 1
    }
  },
  data() {
    return {};
  }
});

</script>

<style lang="scss" scoped>

input, textarea {
  padding: 1rem;
  outline: none;
  border: 2px solid black;
  white-space: normal;
  margin-bottom: 1rem;
  font-family: sans-serif;

  &:focus {
    border-color: red;
  }
}

textarea {
  resize: none; // TODO: Implememt resizing in JS
}
</style>