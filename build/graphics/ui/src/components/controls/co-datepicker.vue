<template lang="pug">
.datepicker
  .view-selector
    coDropdown
    .selection-changer
      coButton &lt; May
      h4 June 2024
      coButton July >
  .date-column-labels
    .label(v-for='label, index in labels' :key='index') #[span {{ label }}]
  .dateSet
    .dates.years(v-if='view === "years"')
    .dates.months(v-if='view === "months"')
    .dates.weeks(v-if='view === "weeks"')
    .dates.days(v-if='view === "days"')
      .date.day(v-for='date, index in dates' :key='index') #[span {{ date }}]
      .selection
</template>

<script lang="ts">

import { defineComponent } from 'vue';
import coDropdown from './co-dropdown.vue';
import coButton from './co-button.vue';

export default defineComponent({
  name: 'datepicker',
  components: {
    coDropdown,
    coButton
  },
  data() {
    return {
      view: 'days',
      labels: ['S', 'M', 'T', 'W', 'T', 'F', 'S'],
      dates: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
    };
  }
});

</script>

<style lang="scss">
.datepicker {
  display: flex;
  flex-direction: column;
  background-color: #ddd;
  width: 100%;
  height: 40rem;
}
.view-selector {
  margin-bottom: 1rem;
}
.selection-changer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.date-column-labels {
  display: grid;
  column-gap: 1rem;
  grid-template-columns: repeat(7, 1fr);
  grid-template-rows: 3rem;
}
.label {
  display: flex;
  align-items: center;
  justify-content: center;
}
.dateSet {
  height: 100%;
}
.dates {
  display: grid;
  row-gap: 1rem;
  column-gap: 1rem;
  height: 100%;
  &.days {
    grid-template-columns: repeat(7, 1fr);
    grid-template-rows: repeat(5, 1fr);
  }
}
.date {
  background: #ccc;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}
// Think we'll have to create a separate item for each seleciton for each row so it can span rows and not be a block
// .selection {
//   background: red;
//   grid-column: 3 / 7;
//   grid-row: 2 /2;
// }
</style>