<script setup>

import {ref} from "vue";
import {invoke} from "@tauri-apps/api";

const nouns=ref([])

invoke("get_nouns").then((res)=>{
  nouns.value=res
})

</script>

<template>
  <div class="mt-2">
    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
      <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th scope="col" class="px-6 py-3">
          Noun
        </th>
        <th scope="col" class="px-6 py-3">
          plural
        </th>
        <th scope="col" class="px-6 py-3">
          definition
        </th>
        <th scope="col" class="px-6 py-3">
          action
        </th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="noun in nouns" class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
          {{noun.noun}}
        </th>
        <td class="px-6 py-4">
          {{noun.plural}}
        </td>
        <td class="px-6 py-4">
          {{noun.definition}}
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