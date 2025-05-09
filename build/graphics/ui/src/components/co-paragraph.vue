<template lang="pug">
p
  slot
</template>

<script lang="ts">
  import { defineComponent } from 'vue';

  export default defineComponent({
    name:'coParagraph',
    data() {
      return {};
    },
  });
</script>