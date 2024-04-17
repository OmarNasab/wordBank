<script setup>

import {invoke} from "@tauri-apps/api";
import {ref} from "vue";
import {useRoute} from "vue-router";

const searchData = ref()
const searchQuery = useRoute().params.query;


invoke("search_all",{query:searchQuery}).then((res)=>{
  console.log(res)
  searchData.value=res
})
</script>

<template>
  <div v-for="(table,index) in searchData">
    <h2>{{index}}</h2>
    <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
      <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
      <tr>
        <th scope="col" class="px-6 py-3" v-for="(_,index) in table[0]">{{index}}</th>
      </tr>
      </thead>
      <tbody>
      <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700" v-for="item in table">
        <td class="px-6 py-4" v-for="value in item">{{value}}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>

</style>