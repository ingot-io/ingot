# Ingot - Authentication IDaaS

Ingot is a robust, high-performance, and customizable authentication infrastructure as a service (IDaaS), designed to handle complex identity management needs for developers. Built with Rust for optimal performance, PostgreSQL for reliable data storage, and Redis for caching and real-time data handling, Ingot enables businesses to integrate secure, scalable, and seamless authentication into their applications.

Ingot provides everything from basic login and signup flows to advanced features like multi-factor authentication, OAuth2, OpenID Connect, social logins, and role-based access control (RBAC). Whether you are building a consumer-facing app or an enterprise-level solution, Ingot is the ideal solution for managing user identities and permissions securely and efficiently.

---

## Key Features

- **Identity and Access Management (IAM):** Full user management with support for role-based access control (RBAC) to define permissions at a granular level.
- **OAuth2 & OpenID Connect (OIDC) Integration:** Native support for OAuth2 and OIDC protocols to enable easy integration with third-party authentication providers.
- **Session Management:** Manage user sessions with advanced features like session expiration, refresh tokens, and multi-session support across devices.
- **Multi-Factor Authentication (MFA):** Enhance security by requiring multiple forms of authentication, such as OTP (One-Time Password), email/SMS verification, or biometric authentication.
- **Role-Based Access Control (RBAC):** Define and enforce access rules based on user roles, ensuring secure resource access and business logic execution.
- **Passwordless Authentication:** Allow users to log in without passwords using magic links or biometric options for enhanced user experience and security.
- **Email & Phone Number Verification:** Secure user onboarding with customizable email and phone number verification flows.
- **Customizable Authentication Flows:** Design user journeys with custom logic and integrations, tailored to your business needs.
- **Scalable & Efficient Architecture:** Built with Rust for speed, PostgreSQL for data persistence, and Redis for caching and real-time data management.
- **Social Login Providers:** Integrate with popular authentication providers such as Google, GitHub, Facebook, Twitter, and more for a seamless login experience.
- **Audit Logs & User Activity Tracking:** Maintain a comprehensive log of user actions, authentication events, and system changes for compliance and security auditing.
- **Delegated Authentication & SSO:** Implement Single Sign-On (SSO) and allow users to authenticate across multiple applications with a unified credential set.
- **API Rate Limiting:** Protect your APIs with built-in rate limiting and throttling to prevent abuse and ensure fair use.
- **Webhooks & Event Notifications:** Integrate with external systems by receiving real-time notifications on authentication events, such as login, registration, password reset, and MFA setup.
- **Granular API Permissions:** Secure and control access to APIs using detailed permissions linked to user roles and groups.
- **Federated Identity Management:** Support for external identity providers via SSO, allowing businesses to connect their internal identity systems to third-party applications.

---

## Architecture Overview

Ingot is designed with modern microservice principles to ensure scalability, security, and performance. The core components of Ingot are:

- **Rust Backend:** The core authentication logic is implemented in Rust for unparalleled performance and concurrency handling.
- **PostgreSQL Database:** All user data, session information, roles, permissions, and audit logs are stored in PostgreSQL for durability and consistency.
- **Redis Caching:** Redis is used to optimize performance by caching frequently accessed data, managing sessions, and improving real-time responsiveness.
- **Protobuf & gRPC API Layer:** Ingot exposes a well-defined, versioned API via gRPC, using Protocol Buffers (Protobuf) for efficient serialization and transport.
  
This architecture provides the flexibility to deploy Ingot on-premise, in a private cloud, or in a containerized environment for maximum scalability.

---

## Getting Started

### Prerequisites

Before setting up Ingot, ensure the following tools and services are available:

- **Rust (1.60+)** – Required for building the application.
- **PostgreSQL (v12+)** – Used as the database for managing user data and authentication records.
- **Redis (v6+)** – Used for caching and real-time session management.
- **Docker** – Optional, but recommended for running Ingot in a containerized environment.

### Installation

#### Clone the Repository

First, clone the Ingot repository to your local machine:

```bash
git clone https://github.com/yourusername/ingot.git
cd ingot
```

Build & Run with Docker
Ingot includes a Docker Compose configuration to simplify deployment. Running the following command will set up the required services (PostgreSQL, Redis) and Ingot itself:

bash
Copy code
```
docker-compose up --build
```

Ingot should now be available at http://localhost:8080 (or a configured port of your choice).

## Apply Database Migrations
Ingot uses Diesel to manage database schema migrations. To apply the initial database setup and other required migrations, run:

## Configuration
You can configure Ingot using environment variables defined in the .env file located in the root of the repository. Some important environment variables to configure:

```
DATABASE_URL=postgres://user:password@localhost:5432/ingot
REDIS_URL=redis://localhost:6379
SECRET_KEY=your-secret-key
MFA_ENABLED=true
SOCIAL_LOGIN_ENABLED=true
```

## API Documentation
Ingot exposes a comprehensive RESTful API based on gRPC and Protocol Buffers for high-performance, versioned service communication. The API allows you to manage everything from user authentication, registration, and session handling to user roles and permissions.

For the full API documentation, refer to the Protobuf definitions located in the proto directory.

- Authentication Service (auth.proto) - Handles login, registration, session management, and password recovery.
- User Service (users.proto) - Manages user details, roles, and permissions.
- Role & Permissions (roles.proto, permissions.proto) - Control access levels and user capabilities within your platform.
- Session Service (sessions.proto) - Manages user sessions, refresh tokens, and session expiration.
- OAuth & SSO (oidc.proto, saml.proto) - Integration with third-party authentication providers.


## Contributing
Ingot is an open-source project, and we welcome contributions from the community. If you would like to contribute, please follow these steps:

- Fork the repository.
- Create a new branch for your feature or bug fix.
- Write tests for the changes you have made.
- Ensure your code follows Rust’s best practices and the project's code style.
- Submit a pull request with a description of your changes.