<script lang="ts">
    export let text: String;

    function animate() {
        let saved_text = text
        // randomize text but keep the same length
        text = text.split("").map(() => String.fromCharCode(Math.floor(Math.random() * 26) + 97)).join("")
        
        // wait 100ms before setting the first character
        // then wait 100ms between each character
        let i = 0
        let interval = setInterval(() => {
            // set the next character
            text = saved_text.slice(0, i) + text.slice(i).split("").map(() => String.fromCharCode(Math.floor(Math.random() * 26) + 97)).join("")
            i++

            // stop the interval when we've reached the end of the string
            if (i > text.length) {
                clearInterval(interval)
           }
        }, 100)
    }

    animate()
</script>

{text}