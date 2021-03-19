<template lang="pug">
//- Need to work in optional progress amount indicators inside and externally for the progress bar
.progress(v-if='bars && range')
  .bar(v-for='(bar, l) in barsGenerated' :key='l' :style='{left: bar[0], width: bar[1]}')
    //- span {{ bar[0] }} {{ bar[1] }} used currently for debugging, will implement good solution for it soon
</template>

<script lang="ts">

/** Convert string to a number */
function convertToNumber(value: string): number {
  // console.log(`Converting ${value} to number`);
  return parseFloat(value);
}

/** Convert number to percentage given our range*/
function convertToPercentage(value: number, range: number): string {
  // console.log(`Converting ${value} to percentage within range (${range})`);
  return value / range * 100 + '%';
}

/** Validate percentage */
function validatePercentage(value: string): boolean {
  if ( !value.match(/^((\d+(\.\d*)?))%$/) ) {
    // console.log(`Value provided is not valid percentage (${value})`)
    return false;
  }
  // console.log(`Value is percentage (${value})`);
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
      type: Array as () => barInterface
    },
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
      // console.log(`range: ${range}`);
      if (this.bars && range) {
        // Loop through the bars, and generate offsets and percentages
        this.bars.forEach( function(bar) {
          // Get length of barsConverted, and use that to set l for index
          const l = barsConverted.length;
          // console.log(`Currently on bar ${l}`);
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
            // console.log(`Checking value ${i} for bar ${l}`);
            switch (typeof value) {
              case 'string':
                // Convert string to number
                currentNumber = convertToNumber(value);
                // Convert currentnumber, if it's provided in a percentage format, to its equvelant is within the current range. 25% of 200 would be 50, where as 25% of 100 would be 25
                if (validatePercentage(value)) {
                  currentNumber = currentNumber / 100 * range;
                  // console.log(`Converted percentage value (${value}) to proper currentNumber (${currentNumber}) based on range (${range})`);
                }
                break;
              case 'number':
                // Store our value in currentNumber
                currentNumber = value;
                break;
            }
            
            // Check if our number is within our range
            if ( !validateRange(currentNumber, [0, range])) {
              // console.log(`Number (${currentNumber}) is not within range (${range})`);
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
              // console.log(`barsConverted before removal: ${barsConverted}`);
              // Remove bar l
              const removedBar = barsConverted.splice(l, 1);
              // console.log(`Errors have been encountered on processing current value (${currentNumber}) of bar ${l}, removed bar ${l} (${removedBar}) from barsConverted (${barsConverted})`);
              // console.log(`Errors encountered: ${errors}`);
            } else {
              // Store the converted / validated percentage in the bars array to be returned
              // console.log(`Storing value ${i} in bar ${l}`);
              barsConverted[l][i] = convertToPercentage(currentNumbers[i], range);
              // console.log(`Stored the percentageOutput (${barsConverted[l][i]}) in barsConverted (${barsConverted})`);
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
  height: 2rem;
  background: #ccc;
  width: 20rem;
  overflow: hidden;
}
.bar {
  position: absolute;
  top: 0;
  width: 0;
  background: rgba(0,0,0, .4);
  height: 100%;
  // transition: left .1s ease, width .3s ease;
}
.rounded-ends {
  border-radius: 100px;
}

</style>