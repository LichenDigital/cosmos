<template lang="pug">
.datepicker
  .view-selector
  .dates
    .years(v-if='view === "years"')
    .months(v-if='view === "months"')
    .weeks(v-if='view === "weeks"')
    .days(v-if='view === "days"') days
</template>

<script lang="ts">

import { defineComponent } from 'vue';

export default defineComponent({
  name: 'datepicker',
  data() {
    return {
      view: 'days'
    };
  }
});

</script>

<style lang="scss">
.datepicker {
  background-color: #ccc;
  width: 100%;
  height: 40rem;
}
</style>