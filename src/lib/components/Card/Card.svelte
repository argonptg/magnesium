<script lang="ts">
  import "$lib/styles/global.scss"
  import "./Card.scss"

  let { title, data = "—" } = $props();
</script>

<div class="card">
  <span class="title">{title}</span>
  <span class="data">{data}</span>
</div>

