<script setup>


import {ref} from "vue";

const formData=ref({
  article:"",
  noun:"",
  plural:"",
  definition:""
})
defineEmits(['save'])

</script>

<template>
  <label>
    <select v-model="formData.article" @input="$emit('save',formData)" class="py-2 px-4 rounded border border-blue-200 w-full">
      <option  selected disabled value="">Artikel</option>
      <option>der</option>
      <option>die</option>
      <option>das</option>
    </select>
  </label>
  <label class="col-span-2">
    <input v-model="formData.noun" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Noun" type="text">
  </label>
  <label class="col-span-2">
    <input v-model="formData.plural" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Plural" type="text">
  </label>
  <label class="col-span-2">
  <input v-model="formData.definition" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="definition" type="text">
</label>

</template>

<style scoped>

</style>