#import "@preview/ilm:1.3.0": *

#set text(lang: "pl")

#let project_name = "Cube Chrono"

#show: ilm.with(
  title: [Specyfikacja wymagań],
  author: [#project_name].text,
  date: datetime(year: 2024, month: 11, day: 12),
  date-format: "[day].[month].[year]r.",
  bibliography: bibliography("refs.bib"),
  chapter-pagebreak: false,
)

#show image: it => {
  align(center, it)
}



= Wprowadzenie


== Cel dokumentu

Dokument stanowi jedyne źródło wymagań aplikacji #project_name. Stanowi podstawę dla specyfikacji oprogramowania. \
Dokument przeznaczony głównie dla zespołu deweloperskiego zajmującego się
wytwarzaniem oprogramowania #project_name.

== Zakres produktu

Celem projektu jest stworzenie oprogramowania wielofunkcyjnego #project_name mającego na celu służyć osobom zainteresowanym kostką Rubika#emoji.tm. System będzie wspomagał liczenie czasów ułożeń, tworzenie statystyk oraz śledzenie postępów.

Składowanie danych w bazie danych umożliwia synchronizację danych między urządzeniami oraz zbieranie statystyk użytkowników i ich analizę.

Aby usprawnić użytkownikom poruszanie się po rankingach oraz szybszą reakcję na zmiany w porównaniu do klasycznych form komunikacji system może zostać wyposażony w mechanizm powiadomień obserwowanych zdarzeń.

== Literatura

@ustawa_ochrona_danych Ustawa z dnia 29 sierpnia 1997 o ochronie danych osobowych (Dz. U. 1997 nr 133 poz. 883 z późn. zm.).


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

#table(
  columns: 2,
  [Procesor], [Intel Xeon Gold 6238T],
  [Pamięć RAM], [128 GB],
  [Przestrzeń dyskowa], [12 TB (licząc 128 MB dla każdego zarejestrowanego użytkownika, 15 MB dla każdego wydarzenia + zapas awaryjny)],
  [System operacyjny], [Linux Kernel > 5.4],
  [Łącze], [10Gbps]
)

== Dokumentacja użytkownika

#table(
  columns: 2,
  [Nazwa], [Instrukcja użytkownika],
  [Opis zawartości], [Opis interfejsu użytkownika/moderatora/administratora oraz jak korzystać z funkcjonalności systemu.],
  [Standard], [Brak],
  [Format], [HTML],
  [Język], [Polski]
)

#table(
  columns: 2,
  [Nazwa], [Specyfikacja interfejsu komunikacyjnego],
  [Opis zawartości], [Opis zawierający opis interfejsu programistycznego aplikacji (API).],
  [Standard], [Brak],
  [Format], [HTML],
  [Język], [Polski]
)

#table(
  columns: 2,
  [Nazwa], [Regulamin systemu],
  [Opis zawartości], [Dokument zawierający regulacje dotyczące korzystania z systemu.],
  [Standard], [Brak],
  [Format], [HTML],
  [Język], [Polski]
)

// == Założenia i zależności



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


// #pagebreak(weak: true)
= Wymagania funkcjonalne

Wymagania funkcjonalne zostały przedstawione na diagramie przypadków użycia. Diagram podzielono na 5 pomniejszych, aby poprawić czytelność.

#image("img/diagrams/Aktorzy.png")
#image("img/diagrams/Gość.png")
#image("img/diagrams/Użytkownik [zalogowany].png")
#image("img/diagrams/Moderator wydarzenia.png", width: 95%)
#image("img/diagrams/Administrator.png", width: 95%)

== Strona główna
== Zarejestruj się
== Zaloguj się
== Profil
== Ranking najlepszych użytkowników
== Strona z wydarzeniami



= Charakterystyka interfejsów


== Interfejs użytkownika

== Interfejsy zewnętrzne
=== Interfejsy komunikacyjne

System udostępnia interfejs programistyczny aplikacji (API), dzięki któremu jest możliwa komunikacja z interfejsem użytkownika. \
System musi pozwalać na dostęp do API innym aplikacją nie stworzonym przez autorów systemu. \
API musi pozwalać na obsługę wszystkich funkcjonalności opisanych w tym dokumencie.


= Wymagania pozafunkcjonalne

#table(
  columns: 2,
  [ID], [*SECURITY*],
  [Nazwa], [*Bezpieczeństwo haseł*],
  [Priorytet], [P0],
  [Opis], [System musi gwarantować bezpieczne przechowywanie haseł. Nie mogą być przechowywane jako tekst jawny.]
)

#table(
  columns: 2,
  [ID], [*NOTIFICATION*],
  [Nazwa], [*Powiadomienia*],
  [Priorytet], [P1],
  [Opis], [System musi być intuicyjny w obsłudze.]
)
