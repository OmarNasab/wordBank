<script setup>

import {ref} from "vue";

const formData=ref({
  verb:"",
  past_simple:"",
  past_perfect:"",
  definition:""
})
defineEmits(["save"]);



</script>

<template>
  <label class="col-span-2">
    <input v-model="formData.verb" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Verb" type="text">
  </label>
  <label class="col-span-2">
    <input v-model="formData.past_simple" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Präteritum" type="text">
  </label>
  <label class="col-span-2">
    <input v-model="formData.past_perfect" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Perfkt" type="text">
  </label>
  <label class="col-span-2">
    <input v-model="formData.definition" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Definition" type="text">
  </label>
</template>

<style scoped>

</style>