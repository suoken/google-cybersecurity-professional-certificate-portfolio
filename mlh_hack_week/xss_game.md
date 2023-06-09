# XSS Game

## Introduction

I've been a professional software engineer for about four years now and I have always been developing web apps and I know how to put a text box on a web application. I've always known about Cross-Site Scripting (XSS) in theory and I know that I always have to sanitize the inputs on a web application to prevent malicious actors from uploading scripts as the payload but it was not until recently I had the ability to play around with actual XSS and hacking. https://xss-game.appspot.com/ by Google is an interactive and fun way to play around with XSS and see the harm it can do to my web applications. Since security is about communication, this document is meant to report what I did during this challenge.

All challenges require you to inject a script to pop up a JavaScript `alert()` so let's get to it. Obviously SPOILERS ahead. Hacking is about breaking and exploring so hack and break and explore the challenge before and only come back here if you need answers.

## Level 1: Hello, world of XSS

<img width="796" alt="Screen Shot 2023-06-08 at 2 44 35 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/b02ad05f-d29e-48d1-8c15-352693883d96">

You're introduced to a web app with a textbox and when you type something into that textbox, it will return "Sorry, no results were found for [your input]. Try again.". Nothing interesting happens. But when you use something like an `<h1>` HTML element into the query, you'll see that word has increased in font size (it turned into a header) and in the url, you'll also see that your element tag was sent in the GET request. So in order to get an `alert()` to show up, you can have your payload contain the `<script>` HTML element. 

Answer: `<script>alert('XSS')</script>`

This is why it's important to sanitize inputs. With no sanitizing, the browser took the payload and thinks it's part of page code and executed it.

## Level 2: Persistence is key.

<img width="796" alt="Screen Shot 2023-06-08 at 4 37 23 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/43da35f6-7790-44dd-be78-2bbaf8b3225a">

You're introduced to a web app that stores your posts. When you put text into the textbox, it shows it on the web app. When you reload the page, you'll see that your message is still there which means that it stores that text into a database and then uploads your text to the web app. So the data has persistence (get it?). Now let's do what we did in level 1, insert an HTML element. If you put in `<h1>Hello, world</h1>`, you'll see that your word also increased in font size which means that the text box doesn't escape the contents of what's in the text box. But try it with a `<script>alert()</script>`, you'll see that nothing shows up. There's now a validation to prevent the `script` element from being used. But since other HTML tags work, you can use one that can take an attribute which will run a function. One attribute that can run a function is the `onerror` attribute.

Answer: `<img src='' onerror='alert('XXS')'>`

Since there is no source for the image, an error will occur which will trigger the `onerror=alert()`. This is why it's important to handle data carefully from server-side databases or client-side caches/local storage. If you reload the page after entering that payload, you'll see the alert again. Sanitize those inputs!

## Level 3: That sinking feeling

<img width="785" alt="Screen Shot 2023-06-08 at 4 38 09 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/a31c8009-7f13-49fc-b005-2ab0870752b6">

No text box this time! But when you interact with this page, you'll see that the URL will change based on what tab you clicked on after a hash (called the URL fragment). So now you know where the door is and where to put a payload. Maybe you can put a `<script>` element there...

Answer: `https://xss-game.appspot.com/level3/frame#3'><script>alert('XSS');</script>'`

But since we just learned about the `onerror` attribute in Level 2, we can also use that.

Alternate answer: `https://xss-game.appspot.com/level3/frame#3' onerror='alert('XSS');`

By right-clicking on the target web app and clicking on 'View Frame Source', you'll see the following JavaScript code.

```javascript
function chooseTab(num) { // Dynamically load the appropriate image
html += "<img src='/static/level3/cloud" + num + ".jpg' />";
$('#tabContent').html(html);
...
window.onload = function() {chooseTab(unescape(self.location.hash.substr(1)) || "1");}
```

So you're calling the `chooseTab` function with `self.location.hash` passed as an argument. So that means you'll need to put your `alert()` payload after the # in the URL. So how can you fool the `chooseTab` function? Since your num payload is in the middle of the `src` attribute, you'll need to close the attribute with a single angle bracket or quote and then add the `alert()` using either method above.

The img tag will look like this at the end and that's how the payload gets incorporated

`<img src='/static/level3/cloud3'><script>alert();</script>.jpg' />`

`<img src='/static/level3/cloud3' onerror='alert('XSS');.jpg' />`

## Level 4: Context matters

<img width="786" alt="Screen Shot 2023-06-08 at 5 45 06 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/dca80e29-5f53-4490-aa96-736360bd3bcc">

You'll see a form to pass a number and that gets passed to a timer page and then after 3 seconds, you're redirected back to the beginning. When you try passing a `<script>alert()</script>`, it doesn't work because the browser doesn't think it's part of the web page (ie. pass an `<h1>Hello</h1>` and it shows the explicit HTML element on the page instead of in bigger font). 

Answer: `3'**alert());//`

So if you enter a random payload and then right click to see 'View Frame Source' when the loading gif shows up, you can see where your payload went to.

```html
<script>
    function startTimer(seconds) {
    seconds = parseInt(seconds) || 3;
    setTimeout(function() { 
        window.confirm("Time is up!");
        window.history.back();
    }, seconds * 1000);
    }
</script>
...
<img src="/static/loading.gif" onload="startTimer('PAYLOAD');" />
```

You can see that the form receives a string and the `startTimer` function turns the string into an int through `parseInt`. You can use this to your advantage by sending the `alert()` via exponentiation on a string. Because JavaScript is weakly typed, it allows for implicit type conversion when an operation involves mismatched types instead of throwing errors. So when performing the exponentiation, JavaScript will try to evaluate `'3'**alert()` before sending it to the `startTimer` function and will just run the `alert()` function hoping the result will be a number. The `//` at the end just comments out the rest of the argument in the function.

```html
<img src="/static/loading.gif" onload="startTimer('3'**alert('XSS'));//');" />
```

This is why you must check for escaping data correctly

## Level 5: Breaking protocol

<img width="809" alt="Screen Shot 2023-06-08 at 6 51 49 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/1bae6fe1-8e18-47e3-9789-ca5f68ed1d55">

The welcome page has a link that leads to the sign up page where you can enter a payload. Sending a `<script>alert('XSS')</script>` does nothing. But inspect the URL and inspect the frame source as well. Notice that the URL changes when going into the signup page. Can you see the URL parameter in the frame source? You can try injecting JavaScript here.

Answer: `https://xss-game.appspot.com/level5/frame/signup?next=javascript:alert('XSS')`

When you View Frame Source on the signup page, you'll see this. 

```html
<a href="confirm">Next >></a>
```

The URL parameter is sent to the `href` attribute in this `a` tag. You can put some JavaScript here using `javascript:alert()` because the browser recognizes this as a bookmarklet.

## Level 6: Follow the 🐇

<img width="799" alt="Screen Shot 2023-06-08 at 6 54 46 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/78b77b66-56e0-4e2c-a172-32518cfdbd7e">

Woah ok nowhere to put inputs. Looks like this a URL operation

Answer: `https://xss-game.appspot.com/level6/frame#data:text/plain,alert('XSS')` 

Take a look at this when you inspect the source code: 

```javascript
// This will totally prevent us from loading evil URLs!
if (url.match(/^https?:\/\//)) {
    setInnerText(document.getElementById("log"),
    "Sorry, cannot load a URL containing \"http\".");
    return;
}

// Load this awesome gadget
scriptEl.src = url;

// Show log messages
scriptEl.onload = function() { 
    setInnerText(document.getElementById("log"),  
    "Loaded gadget from " + url);
}
```

So the big clue is in the beginning where they say find a way to make the application request an external file. It's looking for 'https' to prevent extenal loading but this can be bypassed by omitting the 'http' protocol with `data:`. 

## Done! Cake!

<img width="780" alt="Screen Shot 2023-06-08 at 7 17 14 PM" src="https://github.com/suoken/google-cybersecurity-professional-certificate-portfolio/assets/22568107/85a56439-ff42-42ab-9d03-f8763e5a3da7">


