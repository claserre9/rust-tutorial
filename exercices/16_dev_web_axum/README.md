# Exercices — Chapitre 16 : Axum

## 16.1 — Serveur minimal

Créez un serveur Axum avec ces routes :

```
GET  /             → "Bonjour !"
GET  /echo/:msg    → retourne msg tel quel
POST /majuscules   → corps JSON { "texte": "..." } → retourne { "résultat": "..." } en majuscules
```

Testez avec curl ou httpie.

## 16.2 — État partagé

Ajoutez un compteur de visites partagé entre les routes :

```
GET /count     → retourne le nombre de visites total
```

Chaque requête sur n'importe quelle route incrémente le compteur (middleware).

## 16.3 — CRUD complet (projet cargo)

Étendez le projet api-tasks du cours :

1. Ajoutez une route `GET /tasks/:id` pour récupérer une tâche par ID
2. Ajoutez une route `GET /tasks?fait=true` pour filtrer par statut
3. Ajoutez la validation : description ne peut pas être vide (retourner 422)
4. Ajoutez un test d'intégration avec `axum::test`

```toml
# Pour les tests d'intégration Axum :
[dev-dependencies]
axum-test = "16"
# ou
tower = { version = "0.5", features = ["util"] }
```

## 16.4 — Middleware d'authentification

Implémentez un middleware qui vérifie un header `X-Api-Key`.
Si la clé est absente ou incorrecte, retourner 401.
Appliquez-le uniquement sur les routes `/tasks/*`.
