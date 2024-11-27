---
title: "Python and Junior Developers"
description: "Why Python's simplicity and popularity can be a problem for Junior Developers."
date: "2024-11-09"
tags: ["python", "job market", "programming"]
image: "/static/images/posts/learning-python.webp"
---

I wanted to talk about this ever since I started feeling confident enough to say “I KNOW Python” (a bit cocky of me since I have a lot to learn of course, but you understand the point). As everyone reading this probably knows, Python has an extremely simple syntax, the typical coding example you may show to someone that has never coded before reads just as plain English language in most of it.

## The Problem

What I’ve found is that this provides some so called “Software Engineers” (as self proclaimed in their portfolios and resumes) with the illusion of possessing great programming skills and deserve a job straight away.

What I mean by all of this is that, this is a problem I feel is not addressed often nor well enough, and it is similar to what I like to call “The React myriad” in web development, where you find a lot of “self taught developers” everywhere that pretend to be web devs, wielding their To-Do apps as mighty swords of engineering, without having basic HTML knowledge.

And then you ask this “Python devs” to implement some coding example that is not really that complicated if we are being honest, like some iterator:

```python
class PrimeGenerator:
    def __init__(self):
        self.primes = []  # List to store prime numbers
        self.current = 2  # Start checking from 2 (the first prime number)

    def __iter__(self):
        return self

    def __next__(self) -> int:
        # Find the next prime number
        while True:
            if all(self.current % prime != 0 for prime in self.primes):
                self.primes.append(self.current)
                prime = self.current
                self.current += 1
                return prime
            self.current += 1

    def sieve(self):
        # Generator that yields primes using a sieve-like approach
        for prime in self:
            yield prime
            # Remove multiples of the current prime from further consideration
            self.primes = [p for p in self.primes if p <= prime]

    # Generate the first n primes
    def take(self, n: int) -> list[int]:
        return [next(self) for _ in range(n)]
```

They may not even know what to do, some of them having no clue or previous knowledge of the `yield` keyword existing in Python.

This makes me honestly sad since I love this language for being so versatile and adaptable for many use cases, but still these people give it a bad name making it a toy language or making it seem like it has bad DX (which I understand that to a certain extent is a subjective quality to have) although it not being like this for a lot of people that use it daily.

## YouTube tutorial syndrome

The "YouTube tutorial syndrome" is another facet of this issue. Many aspiring developers rely heavily on video tutorials that often focus on surface-level knowledge without delving into the underlying principles. While these tutorials can be a great starting point, they often create a false sense of mastery. Learners may be able to replicate the code shown in the video, but struggle when faced with simple problems or when asked to explain the thought process that led them to that code. This highlights the need for a more comprehensive learning approach that combines tutorial-based learning with deeper, conceptual understanding and practical application.

## Bridging the Gap: From Syntax to Engineering

I know that in this list everything has been mentioned several times in the software community, but they are not repeated/regarded enough apparently:

- **Emphasize Computer Science Fundamentals:** Encourage learners to study data structures, algorithms, and design patterns. These concepts are language-agnostic and form the backbone of software engineering. Most people do not need to be experts of course, but basic knowledge always comes in handy.
- **Promote Project-Based Learning:** Encourage developers to build complex, real-world projects that go beyond simple scripts or To-Do apps. This hands-on experience is invaluable for understanding software architecture and system design.
- **Foster a culture of Continuous Learning:** The tech industry evolves rapidly. Instill the importance of staying updated with new technologies, best practices, and industry trends.
- **Encourage Code Reviews and Collaboration:** Participate in open-source projects or peer code reviews. This exposure to different coding styles and problem-solving approaches is crucial for growth.

By implementing these strategies, we can help bridge the gap between knowing Python syntax and becoming a proficient software engineer, ensuring that Python's simplicity becomes a stepping stone rather than a stumbling block in a developer's journey.

---

> My contact information is available in [the contact page](/contact) for anyone that would like to reach out to me.
> Thank you for reading this, and come back any time! I will keep posting about my journey and any interesting topic related to software engineering.
>
