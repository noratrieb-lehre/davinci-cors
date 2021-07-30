# Frontend Tests

Am besten 2 Browser verwenden (zb Chrome + Firefox)
Aktionen werden immer mit dem User durchgeführt, der die letzte Aktion durchgeführt hat, ausser es steht explizit
erwähnt.

## Unsicheres Passwort

1. Register a User
    - `hugo@hugo.ch`, `hugo`
    - Fehler, das Passwort muss mindestens 6 Zeichen lang sein

## Klasse erstellen und bearbeiten

1. 2 User Registrieren
    - `hugo@hugo.ch`, `hugo123456` (user1)
    - `hugo@hugo.com`, `hugo123456` (user2)

2. Auf user1 eine Klasse erstellen
    - Name: `beste klasse`, Beschreibung: `unsere klasse`
    - Die Klasse öffnen
    - user1 muss in den Mitgliedern angezeigt werden als 'Eigentümer'

3. user2 einladen
    - Auf dem Admin-Panel von user1 den Klassenlink kopieren
    - Bei user2 den link öffnen
    - user2 kann die Klasse nicht sehen
    - Im Admin-Panel unter Beitrittsanfragen bei user1 user2 akzeptieren
    - Die Seite bei user2 neu laden
    - user2 kann nun die Klasse sehen und ist Mitglied

4. Klasseneinstellungen ändern
    - Mit user1 ins Admin-Panel gehen
    - Unter Mitgliederverwaltung user2 bearbeiten
    - Seinen Namen zu `user2` ändern und die Rolle auf Admin setzen und speichern
    - Zu Einstellungen bei user1 gehen
    - Den Benutzernamen zu `user1` ändern
    - zu Info gehen und die Seite neu laden
    - Die User heissen jetzt `user1` und `user2`, und sind Owner/Admin

5. Mit user2 Klasseneinstellungen ändern
    - Mit user2 ins Admin-Panel gehen
    - Den Klassennamen zu `user2s klasse` ändern
    - Die Beschreibung zu `user1 soll gehen` ändern
    - Unter Info sind die Änderungen zu sehen

6. Owner übertragen und gehen
    - Mit user1 ins Admin-Panel gehen
    - Unter Mitgliederverwaltung user2 bearbeiten
    - user2 zum owner machen
    - Die Klasse verlassen
    - user2 sieht nun sich als owner und keine anderen member

## Stundenplan und wielangenoch

Weiterhin mit der Klasse von user2 arbeiten

1. Stundenplan erstellen
    - Mit user2 ins Admin-Panel gehen zu Lektionenverwaltung
    - Auf Stundenplan erstellen drücken
    - Zu Stundenplan gehen
    - Leere Seite

2. Lektionen hinzufügen
    - Name: `testen`
    - Start: letzte volle Stunde (9:59 -> 9:00)
    - Ende: die übernächste volle Stunde (9:59 -> 11:00)
    - Beschreibung: `ganz viel testen`
    - Tag: der aktuelle Tag
    - Auf Neue Lektion drücken
    - Zu Wie Lange Noch gehen
    - Die Lektion wird angezeigt

## Events

Weiterhin mit der Klasse von user2 arbeiten

1. Event erstellen
    - Zu Admin/Eventverwaltung gehen
    - Name: `testen`
    - Start: Aktueller Tag 2:00
    - Ende: Aktueller Tag: 23:00
    - Beschreibung: `ganz viel testen`
    - Keine Benachrichtigung
    - Prüfung

2. Kalender
    - Zu Kalender gehen
    - `testen` wird angezeigt
    - Auf `testen` drücken
    - Alle Details korrekt angezeigt
    - Auf Event löschen drücken
    - Seite neu laden, zu Kalender gehen
    - Event ist nicht mehr da

## Mitgliederverwaltung

Weiterhin mit der Klasse von user2 arbeiten

1. user1 wieder einladen
    - Den Einladungslink bei user1 einfügen
    - user1 im Adminpanel von user2 ablehnen
    - Den Einladungslink erneut bei user1 einfügen
    - user1 im Adminpanel von user2 akzeptieren

2. user1 bannen
    - Bei user2 ins Adminpanel gehen, zu Mitgliederverwaltung
    - user1 bannen
    - user1 kann nicht mehr auf die Klasse zugreifen
    - Bei user1 den Einladungslink einfügen
    - Er kann nicht beitreten

3. user1 entbannen
    - Bei user2 ins Adminpanel gehen, zu Banns
    - user1 entbannen
    - user1 kann wieder auf die Klasse zugreifen

## Discord

Weiterhin mit der Klasse von user2 arbeiten. Für diesen Abschnitt wird ein [Discord](https://discord.com) Account und
Server benötigt. Dieser sollte separat erstellt werden.

1. Account und Klasse verlinken
    - Bei user2 ins Adminpanel gehen
    - Die Discord-Server ID einfügen und Verbinden.
    - Zu den Accounteinstellungen gehen
    - Die Discord-User ID einfügen und Verbinden.

2. Notifications einrichten
    - Auf den Discord gehen
    - `/settings notification_channel` einen Channel auswählen
    - `/settings notification_role_ping` eine Rolle auswählen
    - `/settings notification_everyone_ping True`

3. Event erstellen
    - Mit user2 zu Admin/Eventverwaltung gehen
    - Name: `testen`
    - Start: Übernächste volle Stunde
    - Ende: Aktueller Tag: 23:00
    - Beschreibung: `ganz viel testen`
    - Benachrichtigung: In 7 Minuten
    - Prüfung
    - Das Event kann nun im Kalender gefunden werden

4. Events suchen auf Discord
    - `/events all`
    - Das Event wird angezeigt
    - `/events next`
    - Das Event wird angezeigt
    - `/events filter Prüfung`
    - Das Event wird angezeigt
    - `/events search tEstEn`
    - Das Event wird angezeigt

5. Auf Notification warten
    - Warten, bis die Notification kommt
    - Sie sollte innerhalb einer Minute auf den Zeitpunkt kommen (In der Backend-Konsole kann, wenn das RUST_LOG Level debug
      ist, nachgeschaut werden, um welche Zeit jeweils die Benachrichtigungen verschickt werden.)
    - Die Rolle und @everyone sollte gepingt werden, und die Beschreibung, Titel und Zeit sollten übereinstimmen  