<template>
  <div class="mt-2">
    <form>
      <div class="grid grid-cols-8 gap-2">
        <label>
          <select class="py-2 px-4 rounded border border-blue-200 w-full" v-model="type" v-on:change="test">
            <option selected disabled value="">Word Type</option>
            <option v-for="(_,form) in availableForms" :value="form" :key="form">{{ form.slice(0,-4) }}</option>
          </select>
        </label>
        <component :is="availableForms[type]"></component>
      </div>
    </form>
  </div>
</template>

<script setup>

import {ref} from "vue";
import NounForm from "../components/NounForm.vue";
import VerbForm from "../components/VerbForm.vue";
import PropositionForm from "../components/PropositionForm.vue";
import PhraseForm from "../components/PhraseForm.vue";
import MedicalPhrases from "./MedicalPhrases.vue";
import MedicalPhrasesForm from "../components/MedicalPhrasesForm.vue";

let type=ref("")

let availableForms= {
  NounForm,
  VerbForm,
  PropositionForm,
  PhraseForm,
  MedicalPhrasesForm
}
function test(){
  console.log(type.value)
}

</script>