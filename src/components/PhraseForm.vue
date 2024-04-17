<script setup>
import {ref} from "vue";

const formData=ref({
  phrase:"",
  definition:"",
  note:""
})

defineEmits(['save'])

</script>

<template>
  <label class="col-span-3">
    <input v-model="formData.phrase" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Phrase" type="text">
  </label>
  <label class="col-span-4">
    <input v-model="formData.definition" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Definition" type="text">
  </label>
  <label class="col-span-8">
    <input v-model="formData.note" @input="$emit('save',formData)" class="py-2 px-4  rounded border border-blue-200 w-full" placeholder="Note" type="text">
  </label>
</template>

<style scoped>

</style>