<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h2>Exercises</h2>
    <ul>
      <li>
        <router-link class="nav-link" :to="{name: 'Day1' }">Day 1</router-link>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  data () {
    return {
      msg: 'Welcome to Advent of Code 2020'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
