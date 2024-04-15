# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?
> Defining the Publisher/Subscriber patterns as an interface allows for a more extensible and scalable codebase, this practice ensures that the codebase is in compliance with the SOLID principles, specifically the Open/Closed principle and the Dependency Inversion principle. By defining the Subscriber as an interface, we can ensure that the codebase is open for extension but closed for modification, as we can add new subscribers without modifying the existing codebase. This also allows for the Dependency Inversion principle to be followed, as the Publisher does not depend on concrete implementations of the Subscriber, but rather on the Subscriber interface. This allows for the codebase to be more modular and easier to maintain in the long run. In this case, our codebase is merely a exemplar without any plans for future extensions, so we can use a single Model struct without any interfaces. However, it is always a good practice to define the Subscriber as an interface to ensure that the codebase is more extensible and scalable in the future.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?
> If the id and url of Subcriber is indeed unique, the usage of DashMap would allow a quicker lookup operation when searching for a particular subscriber, this is more efficient than performing a linear search on a Vec. DashMap allows for O(1) lookup time complexity, while Vec has a time complexity of O(n) for lookup operations. This is especially important when the number of subscribers grows, as the time complexity of a linear search operation grows linearly with the number of subscribers, while the time complexity of a lookup operation in a DashMap remains constant. This is why using DashMap is necessary for this case, as it allows for quicker lookup operations and ensures that the codebase is more efficient and scalable in the long run.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?
> The Singleton pattern is a design pattern that ensures that a class has only one instance and provides a global point of access to that instance. They lazy_static macro handles that for us, so we don't need to implement the Singleton pattern ourselves. Although the Singleton pattern allows for a safe access to the HashMap data, it does not prevent other issues that may arise from concurrent access to the HashMap. The DashMap library provides a safe concurrent access to the HashMap, which is necessary in this case as we are dealing with a shared resource that may be accessed concurrently by multiple threads. So in this case, DashMap and the Singleton pattern goes hand in hand, as the Singleton pattern ensures that there is only one instance of the HashMap preventing constant cloning and initialization, while DashMap ensures that the HashMap is accessed concurrently in a safe manner, therefore in this case, one could not be replaced with another.

#### Reflection Publisher-2
1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?
> The need to separate Service and Repository from a Model is to ensure that the codebase is in compliance with the Single Responsibility Principle (SRP) and the Separation of Concerns principle. The Model in MVC is meant as a data representastion on the database, The Repository is reponsible as a data access and modification layer, while the Service is responsible for the business logic. By separating the Service and Repository from the Model, we can ensure that each component has a single responsibility, and that the codebase is more modular and easier to maintain. This also allows for the codebase to be more extensible and scalable, as we can add new services and repositories without modifying the existing codebase. This separation of concerns also allows for the codebase to be more testable, as we can test each component in isolation. Therefore, the separation of Service and Repository from the Model is necessary to ensure that the codebase is in compliance with the SRP and the Separation of Concerns principle.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?
> If we only use the Model, the codebase would be less modular and harder to maintain. The Model would have to handle both data storage and business logic, which would violate the Single Responsibility Principle (SRP) that could lead to an increase in code complexity and results in a very highly coupled code.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.
> Postman is a very useful tool for testing web endpoints using HTTP requests. Postman allows us to test the endpoints of our web applications without having to write any code, which is very useful for quickly testing the functionality of our web applications. Some features that I really liked from Postman are the ability to save requests and responses, and the ease of controlling the request headers and body and managing cookies. This tool is very helpful for testing the endpoints of our web applications, and I will definitely use it in my future software engineering projects. You could also document your API using Postman, which is very useful for sharing the API documentation with other developers.

#### Reflection Publisher-3
1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?
> On this tutorial, the push model is used. In this model the publisher sends the notification to the subscriber, the subscriber does not need to request the notification from the publisher. This model is used in the notify function in the Notification service, where the publisher sends the notification to the subscriber.

2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)
> The advantages of pull method are that the codebase is more simpler and this model reduces the complexity of the observable. Also the subscriber can choose which data to pull and could decide when to pull the data. The disadvantages of pull method are that the subscriber needs to constantly check the observable for changes, which could be inefficient and could lead to a high CPU usage. Also, the pull method could lead to a high latency in the notification process, as the subscriber needs to constantly check the observable for changes. The push method is more efficient and has a lower latency, as the publisher sends the notification to the subscriber, the subscriber does not need to request the notification from the publisher.

3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.
> This will cause the process to halt whenever we are trying to send a notification to a subscriber. This is because the process will wait for the notification to be sent to the subscriber before continuing to the next process. This will cause the program to be less efficient and could lead to a high latency in the notification process. This problem will escalate when the number of subscribers grows, as the process will have to wait for each subscriber to receive the notification before continuing to the next subscriber. This will cause the program to be less scalable.