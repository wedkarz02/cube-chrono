#import "@preview/ilm:1.3.0": *

#set text(lang: "pl")

#let project_name = "Cube Chrono"

#show: ilm.with(
  title: [Specyfikacja wymagań],
  author: [#project_name].text,
  date: datetime(year: 2024, month: 11, day: 12),
  date-format: "[day].[month].[year]r.",
  bibliography: bibliography("refs.bib", full: true),
  chapter-pagebreak: false,
  figure-index: (enabled: true, title: "Skorowidz Rysunków"),
  table-index: (enabled: true, title: "Skorowidz Tabel"),
  listing-index: (enabled: true)
)

#show image: it => {
  align(center, it)
}

// Wyświetlanie tabel bez napisu
#show figure.where(kind: table): it => [
  #it.body
]



= Wprowadzenie


== Cel dokumentu

Dokument stanowi jedyne źródło wymagań aplikacji #project_name. Stanowi podstawę dla specyfikacji oprogramowania. \
Dokument przeznaczony głównie dla zespołu deweloperskiego zajmującego się wytwarzaniem oprogramowania #project_name.

== Zakres produktu

Celem projektu jest stworzenie oprogramowania wielofunkcyjnego #project_name mającego na celu służyć osobom zainteresowanym kostką Rubika#emoji.tm. System będzie wspomagał liczenie czasów ułożeń, tworzenie statystyk oraz śledzenie postępów.

Składowanie danych w bazie danych umożliwia synchronizację danych między urządzeniami oraz zbieranie statystyk użytkowników i ich analizę.

Aby usprawnić użytkownikom poruszanie się po rankingach oraz szybszą reakcję na zmiany w porównaniu do klasycznych form komunikacji system może zostać wyposażony w mechanizm powiadomień obserwowanych zdarzeń.

== Literatura

@ustawa_ochrona_danych Ustawa z dnia 29 sierpnia 1997 o ochronie danych osobowych (Dz. U. 1997 nr 133 poz. 883).



= Opis ogólny


== Perspektywa produktu
Oprogramowanie #project_name zostanie zaprojektowane jako aplikacja webowa umożliwiająca użytkownikom rejestrowanie oraz analizowanie postępów w treningu układania kostki Rubika. Aplikacja zapewni również funkcjonalności wspierające organizację i moderowanie wydarzeń związanych z tą aktywnością.

#project_name udostępni użytkownikom przyjazny interfejs w formie strony internetowej. System będzie zintegrowany z bazą danych, która będzie przechowywać informacje o użytkownikach, sesjach treningowych oraz aktualnościach dotyczących wydarzeń organizowanych przez moderatorów.

Aplikacja będzie wspierać wizualizację i analizę danych dotyczących postępów użytkowników, a także umożliwiać rejestrację uczestników oraz wprowadzanie wyników przez moderatorów podczas wydarzeń.

== Funkcje produktu
Zakres podstawowych funkcjonalności będzie obejmował: rejestrowanie, zapisywanie, analizę oraz wizualizację wyników zalogowanego użytkownika. 

System będzie również umożliwiał użytkownikom z rolą moderatora tworzenie i edytowanie wydarzeń społecznościowych dla wybranych użytkowników, wprowadzania oraz zatwierdzania ich wyników w systemie.

== Ograniczenia

Liczba zarejestrowanych zawodników w organizacji World Cube Association @world_cube_association to 246130. Zakładając, że około 25% z nich nadal aktywnie trenuje i bierze udział w wydarzeniach, system powinien być zaprojektowany, aby obsłużyć 61531 osób jednocześnie zalogowanych i korzystających z funkcji systemu. Zakładając, że 5% z nich będzie użytkownikiem z rolą moderatora, należy dodać 3076 jednocześnie trwających wydarzeń.

Aby spełniać powyższe wymagania, serwer powinien zawierać taką lub lepszą konfigurację:

#figure(
  table(
    columns: 2,
    [Procesor], [Intel Xeon Gold 6238T],
    [Pamięć RAM], [128 GB],
    [Przestrzeń dyskowa], [12 TB (licząc 128 MB dla każdego zarejestrowanego użytkownika, 15 MB dla każdego wydarzenia + zapas awaryjny)],
    [System operacyjny], [Linux Kernel > 5.4],
    [Łącze], [10Gbps]
  ),
  caption: "Specyfikacja serwera",
) <tab_specyfikacja_serwera>


== Dokumentacja użytkownika

#figure(
  table(
    columns: 2,
    [Nazwa], [Instrukcja użytkownika],
    [Opis zawartości], [Opis interfejsu użytkownika/moderatora/administratora oraz jak korzystać z funkcjonalności systemu.],
    [Standard], [Brak],
    [Format], [HTML],
    [Język], [Polski]
  ),
  caption: "Instrukcja użytkownika",
) <tab_instrukcja_uzytkownika>

#figure(
  table(
    columns: 2,
    [Nazwa], [Specyfikacja interfejsu komunikacyjnego],
    [Opis zawartości], [Opis zawierający opis interfejsu programistycznego aplikacji (API).],
    [Standard], [Brak],
    [Format], [HTML],
    [Język], [Polski]
  ),
  caption: "Specyfikacja interfejsu komunikacyjnego",
) <tab_specyfikacja_interfejsu_komunikacyjnego>

#figure(
  table(
    columns: 2,
    [Nazwa], [Regulamin systemu],
    [Opis zawartości], [Dokument zawierający regulacje dotyczące korzystania z systemu.],
    [Standard], [Brak],
    [Format], [HTML],
    [Język], [Polski]
  ),
  caption: "Regulamin systemu",
) <tab_regulamin_systemu>



= Model procesów biznesowych


== Aktorzy i charakterystyka użytkowników

=== Gość
Gość reprezentuje osobę, która nie posiada konta na platformie lub jeszcze się nie zalogowała. Ma dostęp do czasomierza i może zobaczyć swoje wyniki w aktualnej sesji. Ponadto może zobaczyć stronę z wydarzeniami i rankingami najlepszych graczy.

=== Użytkownik zalogowany
Użytkownik zalogowany to osoba, która pomyślnie zalogowała się na platformie. Posiada wszystkie uprawnienia gościa oraz dodatkowe funkcje, zmiana rodzaju kostki i przegląd wyników zarówno z bieżącej, jak i poprzednich sesji. Może edytować swój profil, sprawdzać swoje miejsce w rankingu oraz listę znajomych. Ma również możliwość zapraszania innych użytkowników do grona znajomych. Ponadto, użytkownik zalogowany może przeglądać swoje statystyki, przeglądać i zapisywać się na wydarzenia, a także tworzyć prywatne wydarzenia, na które zaprasza swoich znajomych.

=== Moderator wydarzenia
Moderator wydarzenia to osoba odpowiedzialna za zarządzanie oficjalnymi wydarzeniami na platformie. Ma uprawnienia do tworzenia, edytowania i usuwania wydarzeń, a także otwierania i zamykania zapisów dla uczestników. Po rozpoczęciu oraz zakończeniu wydarzenia może udostępnić graczom ich statystyki zebrane w trakcie rywalizacji. Moderator posiada także wszystkie uprawnienia przysługujące zalogowanemu użytkownikowi.

=== Administrator
Administrator to osoba z uprawnieniami do zarządzania użytkownikami i oficjalnymi wydarzeniami na platformie. Może wystawiać ostrzeżenia użytkownikom (np. za łamanie regulaminu), zmieniać ich rangę, usuwać konta, a także usuwać graczy z rankingu najlepszych. Administrator ma możliwość usuwania dowolnych oficjalnych wydarzeń oraz rozpatrywania próśb użytkowników o wpisanie do rankingu – może je akceptować lub odrzucać. Administrator posiada także wszystkie uprawnienia przysługujące moderatorowi wydarzenia.


== Obiekty biznesowe

=== Czasomierz
Mierzy czas od momentu puszczenia przycisku do momentu jego ponownego włączenia, co sygnalizuje zakończenie układania kostki.

=== Rodzaje kostki i pozycje startowe
Określa różne rodzaje kostek (np. 3x3x3 lub 5x5x5) i ich pozycji startowych, czyli jak wygląda ułożenie kostki przed rozpoczęciem układania.

=== Historia wyników
Znajdują się w niej wyniki czasomierza zapisane w danych sesjach.

=== Ranking najlepszych użytkowników
Zawiera listę użytkowników, którzy posiadają najlepsze wyniki w danej kategorii. Żeby dodać wynik do rankingu, należy złożyć prośbę.

=== Prośba o dodanie wyniku do oficjalnego rankingu
Składana przez użytkownika w celu dodania jego wyniku do rankingu najlepszych. Administrator może ją odrzucić lub zaakceptować.

=== Oficjalne wydarzenia
Tworzone przez moderatorów wydarzeń. Ustalają oni sposób zapisu - użytkownicy zapisują się własnoręcznie lub akceptując zaproszenie. Moderator może otworzyć i zamknąć zapisy. Po rozpoczęciu oraz zakończeniu wydarzenia może udostępnić graczom ich statystyki zebrane w trakcie rywalizacji.

=== Prywatne wydarzenia
Tworzone przez zalogowanych użytkowników. Użytkownicy zapisują się akceptując wysłane zaproszenie. Po zakończeniu wyświetlane są wyniki.

=== Znajomi
Użytkownicy dodani do listy znajomych danego użytkownika.

=== Profil
Zawiera informacje dotyczące zalogowanego użytkownika, takie jak nazwa użytkownika, zdjęcie profilowe.



= Wymagania funkcjonalne

Wymagania funkcjonalne zostały przedstawione na diagramie przypadków użycia. Diagram podzielono na 5 pomniejszych, aby poprawić czytelność.

#figure(
  image("img/diagrams/Aktorzy.png", height: 33%),
  caption: [Diagram Aktorów],
)
#figure(
  image("img/diagrams/Gość.png"),
  caption: [Diagram Gościa],
)
#figure(
  image("img/diagrams/Użytkownik [zalogowany].png"),
  caption: [Diagram zalogowanego Użytkownika],
)
#figure(
  image("img/diagrams/Moderator wydarzenia.png", width: 90%),
  caption: [Diagram Moderatora wydarzenia],
)
#figure(
  image("img/diagrams/Administrator.png", width: 90%),
  caption: [Diagram Administratora],
)

#set list(spacing: 12pt)
== Strona główna
- *Aktorzy główni:*
  - Wszyscy
- *Aktorzy pomocniczy:*
  - Brak
- *Priorytet:*
  - P0
- *Opis:*
  - Użytkownik otwiera interfejs systemu i widzi stronę główną z wieloma funkcjami do wyboru.
- *Wyzwalacze:*
  - Gość uruchamia interfejs systemu
- *Warunki początkowe:*
  - Brak
- *Warunki końcowe:*
  - Gość widzi zawartość strony głównej
- *Scenariusz główny:*
  - Gość uruchamia interfejs systemu
  - System wyświetla stronę główną zawierającą:
    - Czasomierz
    - Zaloguj się
    - Zarejestruj się
    - Stronę z wydarzeniami
    - Ostatnie czasy
    - Stronę z rankingiem najlepszych graczy
    - Stronę mój profil
    - Listę znajomych
    - Historię wyników z obecnej sesji
- *Scenariusz alternatywny:*
  - Brak
- *Rozszerzenia:*
  - Brak
- *Wyjątki:*
  - Brak
- *Dodatkowe wymagania:*
  - Strony "Mój profil", "Ranking najlepszych graczy" oraz sekcja z listą znajomych i historia wyników z obecnej sesji, są dostępne dopiero po zalogowaniu.
  - Jeśli użytkownik jest już zalogowany, nie wyświetla mu się "Zaloguj się" i "Zarejestruj się".

#set list(spacing: 7pt)
== Zarejestruj się
- *Aktorzy główni:*
  - Gość
- *Aktorzy pomocniczy:*
  - Brak
- *Priorytet:*
  - P0
- *Opis:*
  - Gość nie posiada konta w serwisie i chce się zarejestrować. W tym celu wybiera odpowiednią opcję i podaje dane.
- *Wyzwalacze:*
  - Gość chce utworzyć konto w serwisie. Wybiera opcję rejestracji.
- *Warunki początkowe:*
  - Użytkownik jest niezalogowany.
- *Warunki końcowe:*
  - Konto zostało zarejestrowane w systemie i użytkownik zostaje przekierowany na stronę główną.
- *Scenariusz główny:*
  - Gość wybiera opcję Rejestracja nowego konta
  - Serwis prezentuje formularz rejestracji konta.
  - Gość wypełnia formularz następującymi danymi:
    - e-mail
    - hasło
    - powtórzone hasło
    - nazwa użytkownika
  - Gość zatwierdza
  - Serwis rejestruje nowe konto.
  - Serwis wyświetla potwierdzenie rejestracji konta Gościowi.
  - Użytkownik zostaje przekierowany na stronę główną.
- *Scenariusz alternatywny:*
  - Brak
- *Rozszerzenia:*
  - Brak
- *Wyjątki:*
  - Konto o podanym adresie e-mail jest już zarejestrowane:
    - System wyświetla informacje o duplikacie
    - System wyświetla ponownie formularz rejestracji
  - Hasło oraz powtórzone hasło nie pasują do siebie:
    - System wyświetla informacje o niepasujących hasłach
    - System wyświetla ponownie formularz rejestracji
  - Hasło nie spełnia wymogów bezpieczeństwa:
    - System wyświetla informacje o zbyt słabym haśle
    - System wyświetla ponownie formularz rejestracji
- *Dodatkowe wymagania:*
  - Format adresu e-mail musi być sprawdzany pod względem zgodności z RFC 5322.
  - Hasło musi być sprawdzane czy zawiera przynajmniej 6 znaków, 1 cyfrę, 1 wielką i 1 małą, literę oraz znak specjalny.


#set list(spacing: 12pt)
== Zaloguj się
- *Aktorzy główni:*
  - Gość
- *Aktorzy pomocniczy:*
  - Brak
- *Priorytet:*
  - P0
- *Opis:*
  - Gość posiada konto w serwisie i chce się zalogować używając swojego adresu e-mail lub nazwy użytkownika i hasła.
- *Wyzwalacze:*
  - Gość chce zalogować się do swojego konta w serwisie. Wybiera opcję logowania.
- *Warunki początkowe:*
  - Użytkownik jest niezalogowany.
- *Warunki końcowe:*
  - Użytkownik zalogował się i zostaje przekierowany na stronę główną.
- *Scenariusz główny:*
  - Gość wybiera opcję Zaloguj się.
  - System wyświetla formularz logowania zawierający dwa pola:
    - Adres e-mail/Nazwa użytkownika
    - Hasło
  - Gość wpisuje swój wcześniej zarejestrowany adres e-mail
  - Gość wpisuje swoje wcześniej wybrane hasło do konta.
  - Zatwierdza.
  - System przekierowuje na stronę główną.
  - System wyświetla opcje niedostępne dla użytkownika niezalogowanego.
- *Scenariusz alternatywny:*
  - Brak
- *Rozszerzenia:*
  - Brak
- *Wyjątki:*
  - Gdy Użytkownik wpisze zły login/e-mail lub hasło.
  - System wyświetla informacje o popełnionym błędzie.
  - System wyświetla jeszcze raz formularz logowania.
- *Dodatkowe wymagania:*
  - W przypadku podania błędnego adresu e-mail lub hasła system ze względów bezpieczeństwa nie może informować Gościa które pole formularza zawiera błąd. Komunikat powinien być ogólny. Np. "Błędny login i/lub hasło."


== Profil
- *Aktorzy główni:*
  - Użytkownik [zalogowany], Administrator, Moderator wydarzenia
- *Aktorzy pomocniczy:*
  - Brak
- *Priorytet:*
  - P0
- *Opis:*
  - Zalogowany użytkownik może obejrzeć swój profil zawierający dane personalne i historię wyników z różnych sesji.
- *Wyzwalacze:*
  - Użytkownik jest zalogowany i kliknął opcję "Mój profil".
- *Warunki początkowe:*
  - Użytkownik jest zalogowany.
- *Warunki końcowe:*
  - Użytkownik widzi swoje dane.
- *Scenariusz główny:*
  - Użytkownik wybiera opcję "Mój profil".
  - System wyświetla formularz z profilem użytkownika.
  - System wyświetla historię sesji i wyników.
  - System wyświetla opcję "Edytuj dane".
- *Scenariusz alternatywny:*
  - Brak
- *Rozszerzenia:*
  - Użytkownik może wybrać opcję Edytuj dane w celu edycji swoich danych:
    - System wyświetla dane w formularzu umożliwiającym zmianę.
    - Użytkownik dokonuje zmian. Oraz zatwierdza lub odrzuca zmiany:
      - Gdy Użytkownik zatwierdza zmiany.
      - System aktualizuje dane zmienione przez użytkownika.
    - Gdy Użytkownik odrzuca zmiany:
      - System pokazuje profil użytkownika przed dokonaniem zmian,
- *Wyjątki:*
  - Brak
- *Dodatkowe wymagania:*
  - Brak

\ \ \
#set list(spacing: 10pt)
== Ranking najlepszych użytkowników
- *Aktorzy główni:*
  - Użytkownik [zalogowany], Administrator, Moderator wydarzenia
- *Aktorzy pomocniczy:*
  - Brak
- *Priorytet:*
  - P0
- *Opis:*
  - Zalogowany użytkownik może obejrzeć ranking użytkowników, którzy osiągnęli najlepsze czasy zatwierdzone przez administrację.
- *Wyzwalacze:*
  - Użytkownik jest zalogowany i kliknął opcję "Ranking najlepszych użytkowników".
- *Warunki początkowe:*
  - Użytkownik jest zalogowany.
- *Warunki końcowe:*
  - Użytkownik widzi ranking najlepszych użytkowników.
- *Scenariusz główny:*
  - Użytkownik wybiera opcję "Ranking najlepszych użytkowników".
  - System wyświetla ranking najlepszych użytkowników.
- *Scenariusz alternatywny:*
  - Brak
- *Rozszerzenia:*
  - Użytkownik może kliknąć opcję "Mój ranking", która pokaże jego miejsce w rankingu.
- *Wyjątki:*
  - Brak
- *Dodatkowe wymagania:*
  - Brak


== Strona z wydarzeniami
- *Aktorzy główni:*
  - Użytkownik [zalogowany], Administrator, Moderator wydarzenia
- *Aktorzy pomocniczy:*
  - Brak
- *Priorytet:*
  - P0
- *Opis:*
  - Zalogowany użytkownik może obejrzeć listę aktualnych wydarzeń organizowanych przez moderatorów wydarzeń.
- *Wyzwalacze:*
  - Użytkownik jest zalogowany i kliknął opcję "Wydarzenia".
- *Warunki początkowe:*
  - Użytkownik jest zalogowany.
  \
- *Warunki końcowe:*
  - Użytkownik widzi stronę z aktualnymi wydarzeniami.
- *Scenariusz główny:*
  - Użytkownik wybiera opcję "Wydarzenia".
  - System wyświetla aktualne wydarzenia.
- *Scenariusz alternatywny:*
  - Brak
- *Rozszerzenia:*
  - Moderator wydarzenia i administrator może wybrać opcję "Utwórz wydarzenie":
    - Wyświetla się formularz z danymi wydarzenia.
    - Użytkownik wprowadza dane do formularza:
      - Nazwa wydarzenia
      - Termin wydarzenia
      - Miejsce wydarzenia
      - Sposób zapisu graczy
    - Użytkownik zatwierdza.
  - Moderator wydarzenia i administrator może wybrać opcję "Edytuj wydarzenie" przy wybranym wydarzeniu.
- *Wyjątki:*
  - Brak
- *Dodatkowe wymagania:*
  - Jeżeli moderator wydarzenia chce zmodyfikować wydarzenie, musi być osobą, która je stworzyła.


\ \
= Charakterystyka interfejsów


== Interfejs użytkownika

*Ekran Główny (Dashboard)*

- Wyświetla podstawowe informacje i dostęp do głównych funkcji aplikacji.
- Zawiera czasomierz oraz przyciski "Start/Stop", "Reset" oraz "Scramble".
- Pokazuje aktualny scramble ("mieszanie kostki"), który użytkownik ma ułożyć, oraz historię wyników z poprzednich ułożeń.

*Widok historii wyników*

- Użytkownik może przeglądać swoje wyniki z poprzednich sesji, w tym średnią czasów (np. z 5 i 12 ostatnich ułożeń) oraz najlepsze i najgorsze czasy.
- Możliwość filtrowania wyników i przeglądania statystyk dla różnych sesji lub typów kostek (np. 3x3, 4x4).

\
*Profil użytkownika*

- Dostęp do informacji profilowych użytkownika, takich jak nazwa użytkownika, ulubiona metoda układania (custom default) i osiągnięcia.
- Możliwość edycji danych profilowych oraz ustawień dotyczących sposobu pomiaru czasu (np. start przyciskiem lub klawiszem).

*Widok rankingu i statystyk*

- Strona prezentująca ranking najlepszych wyników użytkowników w różnych kategoriach.
- Pozwala użytkownikowi zgłaszać swoje wyniki do oficjalnego rankingu (po weryfikacji).
- Ranking jest sortowany według średnich czasów lub najlepszych wyników.

*Sekcja wydarzeń*

- Zakładka dla wydarzeń speedcubingowych, z możliwością zapisania się na nadchodzące turnieje i śledzenia wyników.
- Moderatorzy mogą zarządzać wydarzeniami, a użytkownicy - dołączać do prywatnych lub oficjalnych sesji.

\
== Interfejsy zewnętrzne
=== Interfejsy komunikacyjne (API)

Aplikacja udostępnia API pozwalające na interakcję z funkcjami systemu i wymianę danych między aplikacją a klientami zewnętrznymi (np. aplikacje mobilne, narzędzia zewnętrzne do analizy wyników, inne programy trzecie).

*Endpointy:*
- *Autoryzacja*: Endpointy do rejestracji i logowania użytkowników.
- *Czasomierz*: Endpoint do rozpoczęcia i zakończenia pomiaru czasu, co pozwala na dokładne zbieranie wyników.
- *Scramble Generator*: Endpoint do generowania scramble'ów w oparciu o wybrane przez użytkownika parametry (np. typ kostki).
- *Wyniki i statystyki*: Endpointy do zapisu i pobierania wyników oraz obliczania średnich czasów i innych statystyk.
- *Ranking i wydarzenia*: Endpointy do zarządzania wydarzeniami (tworzenie, edycja, zapis uczestników) oraz przeglądania wyników rankingu.

=== Interfejsy bazy danych

- Baza danych, w której zapisywane są wyniki, profile użytkowników, historie ułożeń oraz dane o wydarzeniach. Komunikacja z bazą odbywa się przez warstwę backendową.
- System zapewnia zabezpieczenia dostępu do bazy oraz chroni dane użytkowników przed nieautoryzowanym dostępem.


#pagebreak(weak: true)
= Wymagania pozafunkcjonalne

#figure(
  table(
    columns: 2,
    [ID], [*SECURITY-01*],
    [Nazwa], [*Bezpieczeństwo haseł*],
    [Priorytet], [P0],
    [Opis], [System musi gwarantować bezpieczne przechowywanie haseł. Nie mogą być przechowywane jako tekst jawny.]
  ),
  caption: [Bezpieczeństwo haseł]
)

#figure(
  table(
    columns: 2,
    [ID], [*SECURITY-02*],
    [Nazwa], [*Ochrona danych osobowych*],
    [Priorytet], [P0],
    [Opis], [System musi spełniać wymagania prawne dotyczące ochrony danych (np. RODO), zapewniając bezpieczne przechowywanie i przetwarzanie danych osobowych użytkowników oraz informując ich o polityce prywatności]
  ),
  caption: [Ochrona danych osobowych]
)

#figure(
  table(
    columns: 2,
    [ID], [*SECURITY-03*],
    [Nazwa], [*Bezpieczna autoryzacja*],
    [Priorytet], [P1],
    [Opis], [System powinien oferować opcjonalne uwierzytelnianie dwuskładnikowe (2FA) lub inne metody wzmacniające bezpieczeństwo logowania.]
  ),
  caption: [Bezpieczna autoryzacja]
)

#figure(
  table(
    columns: 2,
    [ID], [*PERFORMANCE-01*],
    [Nazwa], [*Szybkość działania*],
    [Priorytet], [P0],
    [Opis], [System musi odpowiadać na akcje użytkownika, takie jak generowanie scramble'ów czy zapis wyników, w czasie nie dłuższym niż 1 sekunda, aby zapewnić płynność obsługi.]
  ),
  caption: [Szybkość działania]
)

#figure(
  table(
    columns: 2,
    [ID], [*PERFORMANCE-02*],
    [Nazwa], [*Skalowalność*],
    [Priorytet], [P1],
    [Opis], [Aplikacja powinna być w stanie obsłużyć do 61531 jednocześnie zalogowanych użytkowników oraz do 3076 aktywnych wydarzeń bez zauważalnych spadków wydajności.]
  ),
  caption: [Skalowalność]
)

#pagebreak(weak: true)

#figure(
  table(
    columns: 2,
    [ID], [*AVAILABILITY*],
    [Nazwa], [*Ciągłość działania*],
    [Priorytet], [P1],
    [Opis], [System powinien być dostępny 99% czasu w skali miesiąca, z planowanymi przerwami konserwacyjnymi ogłaszanymi z wyprzedzeniem użytkownikom.]
  ),
  caption: [Ciągłość działania]
)

#figure(
  table(
    columns: 2,
    [ID], [*USABILITY-01*],
    [Nazwa], [*	Intuicyjność interfejsu*],
    [Priorytet], [P0],
    [Opis], [	Interfejs użytkownika powinien być prosty i intuicyjny, umożliwiając użytkownikom szybki dostęp do najważniejszych funkcji, takich jak czasomierz i historia wyników.]
  ),
  caption: [Intuicyjność interfejsu]
)

#figure(
  table(
    columns: 2,
    [ID], [*USABILITY-02*],
    [Nazwa], [*Dostępność językowa*],
    [Priorytet], [P2],
    [Opis], [System powinien oferować obsługę języków, takich jak polski i angielski, aby użytkownicy mogli korzystać z aplikacji w preferowanym języku.]
  ),
  caption: [Dostępność językowa]
)

#figure(
  table(
    columns: 2,
    [ID], [*COMPATIBILITY-01*],
    [Nazwa], [*Wsparcie przeglądarek*],
    [Priorytet], [P1],
    [Opis], [Aplikacja musi działać poprawnie na najnowszych wersjach popularnych przeglądarek, takich jak Chrome, Firefox, Safari i Edge.]
  ),
  caption: [Wsparcie przeglądarek]
)

#figure(
  table(
    columns: 2,
    [ID], [*COMPATIBILITY-02*],
    [Nazwa], [*Zgodność z urządzeniami mobilnymi*],
    [Priorytet], [P1],
    [Opis], [	System powinien być responsywny i w pełni funkcjonalny na urządzeniach mobilnych, zapewniając łatwość obsługi na mniejszych ekranach.]
  ),
  caption: [Zgodność z urządzeniami mobilnymi]
)

#figure(
  table(
    columns: 2,
    [ID], [*NOTIFICATION*],
    [Nazwa], [*Powiadomienia*],
    [Priorytet], [P1],
    [Opis], [System musi być intuicyjny w obsłudze.]
  ),
  caption: [Powiadomienia]
)
