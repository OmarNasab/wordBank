<script setup lang="ts">

import {ref} from "vue";

const isHidden = ref(true);

</script>

<template>
  <button @click="isHidden=false" class="bg-blue-500 hover:bg-blue-800 text-white text-xl px-4 py-2 rounded-lg">
    <slot name="button_text"></slot>
  </button>
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50" :class="isHidden?'hidden':''" @click.self="isHidden=true">
    <div class="w-1/2 p-6 bg-white rounded-lg">
      <slot></slot>
    </div>
  </div>

</template>

<style scoped>

</style>