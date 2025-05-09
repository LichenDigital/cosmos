<template lang="pug">
//- Need to work in optional progress amount indicators inside and externally for the progress bar
.progress(v-if='bars && range')
  .background(
    :style='{background: progressBackground}'
    @click='$emit("selectElement", {element: "progress", event: $event})')
  .bar(
    v-for='(bar, l) in barsGenerated'
    :key='l'
    :style='{left: bar[0], width: bar[1]}'
    @click='$emit("selectElement", {element: "bar", bar: l, event: $event})')
    //- span {{ bar[0] }} {{ bar[1] }} used currently for debugging, will implement good solution for it soon
</template>

<script lang="ts">

import { log } from '../cosmos';

/** Convert string to a number */
function convertToNumber(value: string): number {
  log('extraverbose', `Converting ${value} to number`);
  return parseFloat(value);
}

/** Convert number to percentage given our range*/
function convertToPercentage(value: number, range: number): string {
  log('extraverbose', `Converting ${value} to percentage within range (${range})`);
  return value / range * 100 + '%';
}

/** Validate percentage */
function validatePercentage(value: string): boolean {
  if ( !value.match(/^((\d+(\.\d*)?))%$/) ) {
    log('extraverbose', `Value provided is not valid percentage (${value})`)
    return false;
  }
  log('extraverbose', `Value is percentage (${value})`);
  return true;
}

/** Validate that value is within range */
function validateRange(value: number, range: [number, number]): boolean {
  if ( value < range[0] || value > range[1]) {

    return false;
  } else {
    return true;
  }
}

/** Caluclate diff */
function calculateDiff(value1: number, value2: number): number {
  return value2 - value1;
}

// TODO: Implement the adjustment following (allowing for a general setting, and individual overrides)
// - height of the bars
// - height of the progress element
// - color of the bars / progress
// - border style of the bars / progress
// - corner / end conditions of the bars (rounded or round or square)
// - provide a slot for inside the bars
// - provide a slot for inside the progress
// - think about having some sort of ability to replace the bar with an SVG so that it can be animated (think of a giggly bar)
// I think that the solution to individual bar manipulation / styling may lie in having an internal list of bars, so that bars can be provided in the prop as either an array with a start and end point, or an object with a start, end, and style property etc. That way if the user can keep it super simple and pass in an array of data, with the style for the bars being dictated globally, but they can also override the style of each bar if they pass in an object instead of an array with extra info


import { defineComponent } from 'vue';

type barInterface = (string | number)[][];

export default defineComponent({
  name: 'coProgress',
  props: {
    range: {
      type: Number,
      required: true
    },
    bars: {
      // TODO: Create internal array to store state in percentages and emit those when changed...
      type: Array as () => barInterface
    },
    progressBackground: {
      type: String
    },
    // FIXME: figure out why the default is interfering with the other props for some reason....
    // progressStyle: {
    //   type: Object,
    //   default() {
    //     return { background: '#f00' };
    //   },
    // },
    // TODO: Implement direction
    // direction: {
    //   type: String,
    //   default: 'forward'
    // }
  },
  data() {
    return {
      // TODO: Use a style object instead (probably a computed property instead)
      // style: {}
    }
  },
  computed: {
    barsGenerated(): string[][] {
      // TODO: Figure out if this should be used directly in the template or if it should store the results and then use those stored results in the template
      // TODO: Need to figure out if we should normalize the bars, like this does https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered so that they are ordered, don't overlap, aren't empty, and don't touch (adjacent ranges are folded into a bigger range)?
      
      // Create bars array to store the bars we're generating
      const barsConverted: string[][] = [];
      // Set range to be expected as a number
      const range: number = this.range;
      log('extraverbose', `range: ${range}`);
      if (this.bars && range) {
        // Loop through the bars, and generate offsets and percentages
        this.bars.forEach( function(bar) {
          // Get length of barsConverted, and use that to set l for index
          const l = barsConverted.length;
          log('extraverbose', `Currently on bar ${l}`);
          // // Assign empty array to bar l
          barsConverted[l] = [];
          // Create an array to store our intermediary numbers
          const currentNumbers: [number, number] = [0,0];
          // Store error
          let errorCaught = false;
          const errors: string[] = [];
          
          // Check number of values in each bar. Min 1, Max 2
          if ( !validateRange(bar.length, [1,2]) ) {
            errorCaught = true;
            errors.push(`Too many or too few values (${bar.length})`);
          }
          // If bar only has one value, than add the starting position defaulting to 0
          if ( bar.length === 1 ) {
            bar.unshift(0);
          }

          // For each bar, loop through its children and generate our left offset and width
          bar.forEach( function(value, i) {
            // Create a variable to store the intermediary number
            let currentNumber: number;

            // Check if number or string
            log('extraverbose', `Checking value ${i} for bar ${l}`);
            switch (typeof value) {
              case 'string':
                // Convert string to number
                currentNumber = convertToNumber(value);
                // Convert currentnumber, if it's provided in a percentage format, to its equvelant is within the current range. 25% of 200 would be 50, where as 25% of 100 would be 25
                if (validatePercentage(value)) {
                  currentNumber = currentNumber / 100 * range;
                  log('extraverbose', `Converted percentage value (${value}) to proper currentNumber (${currentNumber}) based on range (${range})`);
                }
                break;
              case 'number':
                // Store our value in currentNumber
                currentNumber = value;
                break;
            }
            
            // Check if our number is within our range
            if ( !validateRange(currentNumber, [0, range])) {
              log('extraverbose', `Number (${currentNumber}) is not within range (${range})`);
              errorCaught = true;
              errors.push(`currentNumber (${currentNumber}) is outside our range (${range})`);
            }

            // Check which number we're dealing with
            switch (i) {
              case 0:
                // If it's our first, store it's value
                currentNumbers[i] = currentNumber;
                break;
              case 1:
                // If it's our second
                // Check to make sure it's larger or equal than our first number
                if ( !(currentNumber >= currentNumbers[0]) ) {
                  errorCaught = true;
                  errors.push(`Our second number (${currentNumber}) is not larger than or equal to our first (${currentNumbers[0]})`);
                }
                // Calculate the difference between them, which will become our width
                currentNumbers[i] = calculateDiff(currentNumbers[0], currentNumber);
                break;
            }

            // Check if we encountered any errors
            if (errorCaught) {
              // Check bars before removal
              log('extraverbose', `barsConverted before removal: ${barsConverted}`);
              // Remove bar l
              const removedBar = barsConverted.splice(l, 1);
              log('extraverbose', `Errors have been encountered on processing current value (${currentNumber}) of bar ${l}, removed bar ${l} (${removedBar}) from barsConverted (${barsConverted})`);
              log('extraverbose', `Errors encountered: ${errors}`);
            } else {
              // Store the converted / validated percentage in the bars array to be returned
              log('extraverbose', `Storing value ${i} in bar ${l}`);
              barsConverted[l][i] = convertToPercentage(currentNumbers[i], range);
              log('extraverbose', `Stored the percentageOutput (${barsConverted[l][i]}) in barsConverted (${barsConverted})`);
            }
          })
        });
        return barsConverted;
      } else {
        return [];
      }
      
    }
  }
})
</script>

<style lang="scss" scoped>

.progress {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 1rem;
}
.background {
  height: 100%;
  width: 100%;
  background: #ccc;
}
.bar {
  position: absolute;
  top: 0;
  width: 0;
  background: rgba(0,0,0, .4);
  height: 100%;
  // height: 50%;
  // transform: translateY(-50%);
  // pointer-events: none;
  // transition: left .1s ease, width .3s ease;
}
.rounded-ends {
  border-radius: 100px;
}

</style>