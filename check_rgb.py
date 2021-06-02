#!/usr/bin/python3

import requests
import sys
import arrow
from ics import Calendar

URI = 'https://studiorenegade.fr/agenda-5.ics'

r = requests.get(URI)

if r.status_code != 200:
    print('''UNKNOWN: Impossible d'obtenir le planning RGB: error code=%s)''' % r.status_code)
    sys.exit(3)

buf = r.text

try:
    calendar = Calendar(buf)
except Exception as exp:
    print('''ERROR: Impossible de parser le fichier ics: %s''' % exp)
    sys.exit(2)

timeline = list(calendar.timeline)
if len(timeline) == 0:
    print('''Pas de RGB plannifié actuellement :'( ''')
    sys.exit(3)

next_event = timeline[0]

begin = next_event.begin
now = arrow.utcnow()

date_str = begin.to('local').format('dddd DD MMM YYYY à HH:mm', locale='fr')

diff = begin - now
nb_days = diff.days

if nb_days <= 1:
    print('''<span style='color:red'>ALERTE 🎉</span>: le prochain RGB est quasiment là! Tenez vous prêt! [ <span style='color:purple'>%s</span> ]''' % (date_str))
    sys.exit(2)

if nb_days <= 3:
    print('''<span style='color:orange'>ATTENTION ❗</span>: le prochain RGB est très bientôt! (dans %s jours) Réservez votre soirée! [ <span style='color:purple'>%s</span> ]''' % (nb_days, date_str))
    sys.exit(1)

if nb_days >= 7:
    print('''<span style='color:green'>OK</span>: Le prochain RGB est encore loin (dans %s jours), vous avez le temps [ <span style='color:purple'>%s</span> ]''' % (nb_days, date_str))
    sys.exit(0)
