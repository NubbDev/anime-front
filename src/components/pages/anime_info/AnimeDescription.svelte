<script lang="ts">
    export let description: string;

    let height: "show" | "hide" = "hide";
</script>

<div class="description">
    <h3>Overview</h3>
    <p class="{height}">{@html description}</p>
    <button on:click={() => height = height === "show" ? "hide" : "show"}>{height === "hide" ? "Read more" : "Read less"}</button>
</div>

<style>
    
    .description {
        margin: 0;
        padding: 0 2rem;
        background-color: transparent;
        border-radius: 1rem;
        box-sizing: border-box;
        overflow: hidden;
        
        position: relative;
    }
    
    button {
        display: block;
        background: transparent;
        border: none;
        margin: 0 auto;
        color: var(--primary);
        font-size: 1rem;
        cursor: pointer;
        font-weight: 700;
    }

    h3 {
        font-size: 1.5rem;
        margin-top: 0.25rem;
        margin-bottom: 1rem;
    }

    p {
        font-size: 1rem;
        position: relative;
        overflow: hidden;
        transition: max-height 0.5s ease-out;
    }
    p.show {
        max-height: 500vh;
    }
    p.hide {
        max-height: 15vh;
    }
    p::after {
        content: "";
        position: absolute;
        bottom: 0;
        right: 0;
        width: 100%;
        background: linear-gradient(180deg, #00000000 0%, var(--background-secondary) 95%);
        transition: height 0.5s ease-out, opacity 0.5s ease-out;
    }
    p.hide::after {
        height: 10vh;
        opacity: 1;
    }
    p.show::after {
        height: 10vh;
        opacity: 0;
    }
    
</style>