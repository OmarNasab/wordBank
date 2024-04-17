<script setup>

import {ref} from "vue";
import {invoke} from "@tauri-apps/api";

const medical_phrases=ref([])

invoke("get_medical_phrases").then((res)=>{
  medical_phrases.value=res
})

</script>

<template>
  <div class="mt-2">
    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
      <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th scope="col" class="px-6 py-3">
          Phrase
        </th>
        <th scope="col" class="px-6 py-3">
          definition
        </th>
        <th scope="col" class="px-6 py-3">
          note
        </th>
        <th scope="col" class="px-6 py-3">
          action
        </th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="phrase in medical_phrases" class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
          {{phrase.phrase}}
        </th>
        <td class="px-6 py-4">
          {{phrase.definition}}
        </td>
        <td class="px-6 py-4">
          {{phrase.note}}
        </td>
        <td class="px-6 py-4">
          view
        </td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>

</style>